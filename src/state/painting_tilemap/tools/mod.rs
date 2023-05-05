mod tile_painter;
mod tile_eraser;
mod tile_whois;
mod tile_picker;

use bevy::{prelude::*, ecs::query::WorldQuery};
use bevy_editor_pls::egui::{self, Painter};

use crate::tile_id_to_pos;

use super::{*, queries::TilemapPoints};

pub use tile_painter::TilePainter;
pub use tile_eraser::TileEraser;
pub use tile_whois::TileWhoIs;
pub use tile_picker::TilePicker;

#[derive(WorldQuery)]
#[world_query(mutable)]
pub struct TilePropertyQuery {
    color: &'static mut TileColor,
    flip: &'static mut TileFlip,
    texture: &'static mut TileTextureIndex,
}

#[derive(Clone, Copy, Debug)]
pub struct TileProperties {
    pub color: TileColor,
    pub flip: TileFlip,
    pub texture: TileTextureIndex,
}

impl Default for TileProperties {
    fn default() -> Self {
        Self {
            color: TileColor(Color::WHITE),
            flip: TileFlip::default(),
            texture: TileTextureIndex(0),
        }
    }
}

pub struct ToolContext<'w, 's> {
    world: &'w mut World,
    points: TilemapPoints,
    tilemap_entity: Entity,
    tilemap_texture_egui: egui::TextureId,
    query: &'s mut QueryState<TilePropertyQuery, ()>,
    pub brush_state: &'s mut TileProperties,
}

impl<'w, 's> ToolContext<'w, 's> {
    pub fn new(
        world: &'w mut World,
        points: TilemapPoints,
        tilemap_entity: Entity,
        tilemap_texture_egui: egui::TextureId,
        query: &'s mut QueryState<TilePropertyQuery, ()>,
        brush_state: &'s mut TileProperties,
    ) -> Self {
        Self {
            world,
            points,
            tilemap_entity,
            tilemap_texture_egui,
            query,
            brush_state,
        }
    }

    pub fn get_tile(&self, pos: TilePos) -> Option<Entity> {
        let tilemap_entity = self.world.get_entity(self.tilemap_entity)
            .expect("Bad tilemap ID");
        let storage = tilemap_entity.get::<TileStorage>()
            .expect("Tilemap has no storage");

        storage.get(&pos)
    }

    pub fn despawn_tile(
        &mut self,
        pos: TilePos,
    ) {
        let Some(tile_entity) = self.get_tile(pos) else { return; };

        self.world.despawn(tile_entity);
        self.world.get_mut::<TileStorage>(self.tilemap_entity)
            .expect("Tilemap has no storage")
            .remove(&pos);
    }

    pub fn set_tile_properties(
        &mut self,
        pos: TilePos,
        props: TileProperties,
    ) {
        let tile_entity = match self.get_tile(pos) {
            Some(x) => x,
            None => {
                let tile_entity = self.world.spawn(TileBundle {
                    tilemap_id: TilemapId(self.tilemap_entity),
                    position: pos,
                    ..default()
                }).id();

                self.world.get_mut::<TileStorage>(self.tilemap_entity)
                    .expect("Tile storage not found")
                    .set(&pos, tile_entity);

                tile_entity
            },
        };
        let mut props_item = self.query.get_mut(&mut self.world, tile_entity)
            .expect("Bad tile entity");

        *props_item.color = props.color;
        *props_item.flip = props.flip;
        *props_item.texture = props.texture;
    }

    pub fn get_tile_properties(
        &self,
        pos: TilePos,
    ) -> Option<TileProperties> {
        let tile_entity = self.get_tile(pos)?;
        let props_item = self.query.get_manual(&self.world, tile_entity)
            .expect("Bad tile entity");

        Some(TileProperties {
            color: *props_item.color,
            flip: *props_item.flip,
            texture: *props_item.texture,
        })
    }

    pub fn tile_rect(
        &self,
        pos: TilePos,
    ) -> egui::Rect {
        let sample_rect = self.points.grid_sample_rect();
        let sample_size = sample_rect.size();
        let offset = egui::vec2(
            pos.x as f32 * sample_size.x,
            // Flip the Y axis, because the TilePos goes up
            // BUT egui Y goes down
            -(pos.y as f32) * sample_size.y,
        );

        sample_rect.translate(offset)
    }

    fn brush_mesh(
        &self,
        rect: egui::Rect,
        uv: egui::Rect,
    ) -> egui::Shape {
        let [r, g, b, a] = self.brush_state.color.0.as_rgba_f32();
        let color = egui::Color32::from_rgba_unmultiplied(
            (r * 255.0) as u8,
            (g * 255.0) as u8,
            (b * 255.0) as u8,
            (a * 255.0) as u8,
        );
        let mut mesh = egui::Mesh::with_texture(self.tilemap_texture_egui);

        mesh.indices.extend([
            0, 1, 2,
            0, 2, 3,
        ]);
        mesh.vertices.extend([
            egui::epaint::Vertex {
                color,
                pos: rect.left_top(),
                uv: uv.left_top(),
            },
            egui::epaint::Vertex {
                color,
                pos: rect.right_top(),
                uv: uv.right_top(),
            },
            egui::epaint::Vertex {
                color,
                pos: rect.right_bottom(),
                uv: uv.right_bottom(),
            },
            egui::epaint::Vertex {
                color,
                pos: rect.left_bottom(),
                uv: uv.left_bottom(),
            },
        ]);


        let trans = rect.center().to_vec2();

        // Undo translate
        mesh.translate(-trans);

        if self.brush_state.flip.d {
            mesh.rotate(
                egui::emath::Rot2::from_angle(std::f32::consts::FRAC_PI_2),
                egui::Pos2::ZERO,
            )
        }

        // Combine x and y flips into negative scaling
        let mut scale = egui::vec2(
            1.0 - 2.0 * self.brush_state.flip.x as u8 as f32,
            1.0 - 2.0 * self.brush_state.flip.y as u8 as f32,
        );

        // multiply that scale by d
        if self.brush_state.flip.d {
            scale.x *= -1.0;
        }

        // Apply the scale
        mesh.vertices.iter_mut().for_each(|v| v.pos = egui::pos2(
            scale.x * v.pos.x,
            scale.y * v.pos.y,
        ));

        // Reapply translate
        mesh.translate(trans);

        egui::Shape::mesh(mesh)
    }

    pub fn paint_tile(
        &self,
        ui_rect: egui::Rect,
        painter: &Painter,
    ) {
        painter.add(self.brush_mesh(
            ui_rect,
            self.tile_info(self.brush_state.texture.0),
        ));
    }

    fn tile_info(
        &self,
        id: u32,
    ) -> egui::Rect {
        let tilemap_entity = self.world.get_entity(self.tilemap_entity)
            .expect("Bad tilemap ID");
        let tilemap_texture = tilemap_entity.get::<TilemapTexture>()
            .expect("Tilemap without texture");
        let tilemap_tile_size = tilemap_entity.get::<TilemapTileSize>()
            .expect("Tilemap without texture");
        match &tilemap_texture {
            TilemapTexture::Single(x) => {
                let tile_size = bevy_to_egui(tilemap_tile_size.into());
                let atlas_size = self.world.resource::<Assets<Image>>().get(x)
                    .expect("Bad image handle")
                    .size();
                let pos = tile_id_to_pos(id, bevy_to_egui(atlas_size), tile_size);
                let uv = egui::Rect::from_min_size(pos, tile_size);

                egui::Rect {
                    min: egui::pos2(uv.min.x / atlas_size.x, uv.min.y / atlas_size.y),
                    max: egui::pos2(uv.max.x / atlas_size.x, uv.max.y / atlas_size.y),
                }
            },
            TilemapTexture::Vector(_) => todo!(),
            TilemapTexture::TextureContainer(_) => todo!(),
        }
    }
}

pub trait Tool: std::fmt::Debug + Send + Sync {
    fn name(&self) -> &'static str;

    fn viewport_ui(
        &mut self,
        ctx: &mut ToolContext,
        hovered_tile: TilePos,
        ui: &mut egui::Ui,
    );
}
