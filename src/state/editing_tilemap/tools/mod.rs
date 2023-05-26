mod tile_painter;
mod tile_eraser;
mod tile_whois;
mod tile_picker;

use bevy::prelude::*;
use bevy_editor_pls::egui::{self, Painter};
use crate::queries::{ TilePropertyQuery, TilemapPoints, TilemapQuery };
use crate::coord_utils::tile_id_to_pos;

use super::*;

pub use tile_painter::TilePainter;
pub use tile_eraser::TileEraser;
pub use tile_whois::TileWhoIs;
pub use tile_picker::TilePicker;

pub type Result<T> = core::result::Result<T, EditorError>;

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
    tile_query: &'s mut QueryState<TilePropertyQuery, ()>,
    tilemap_query: &'s mut QueryState<TilemapQuery, ()>,
    tile_data: Option<&'s mut HashMap<u32, TileData>>,
    brush_state: &'s mut TileProperties,
}

impl<'w, 's> ToolContext<'w, 's> {
    pub(crate) fn new(
        world: &'w mut World,
        points: TilemapPoints,
        tilemap_entity: Entity,
        tilemap_texture_egui: egui::TextureId,
        tile_query: &'s mut QueryState<TilePropertyQuery, ()>,
        tilemap_query: &'s mut QueryState<TilemapQuery, ()>,
        tile_data: Option<&'s mut HashMap<u32, TileData>>,
        brush_state: &'s mut TileProperties,
    ) -> Self {
        Self {
            world,
            points,
            tilemap_entity,
            tilemap_texture_egui,
            tile_query,
            tilemap_query,
            tile_data,
            brush_state,
        }
    }

    pub fn get_tile(&self, pos: TilePos) -> Result<Option<Entity>> {
        let storage = self.tilemap_query.get_manual(&self.world, self.tilemap_entity)
            .map_err(|query_error| EditorError::BadTilemapEntity {
                tilemap_entity: self.tilemap_entity,
                query_error,
            })?
            .storage;

        Ok(storage.get(&pos))
    }

    pub fn despawn_tile(
        &mut self,
        pos: TilePos,
    ) -> Result<()> {
        let Some(tile_entity) = self.get_tile(pos)? else { return Ok(()); };

        if !self.world.despawn(tile_entity) {
            warn!(
                "The tile entity ID was present at {pos:?} for tilemap {:?}, but it was an invalid entity ID",
                self.tilemap_entity,
            )
        }

        self.tilemap_query.get_mut(&mut self.world, self.tilemap_entity)
            .map_err(|query_error| EditorError::BadTilemapEntity {
                tilemap_entity: self.tilemap_entity,
                query_error,
            })?
            .storage
            .remove(&pos);

        Ok(())
    }

    pub fn set_tile_properties(
        &mut self,
        tile_pos: TilePos,
        props: TileProperties,
    ) -> Result<()> {
        let tile_entity = match self.get_tile(tile_pos)? {
            Some(x) => x,
            None => {
                let tile_entity = self.world.spawn(TileBundle {
                    tilemap_id: TilemapId(self.tilemap_entity),
                    position: tile_pos,
                    ..default()
                }).id();

                self.tilemap_query.get_mut(&mut self.world, self.tilemap_entity)
                    .map_err(|query_error| EditorError::BadTilemapEntity {
                        tilemap_entity: self.tilemap_entity,
                        query_error,
                    })?
                    .storage
                    .set(&tile_pos, tile_entity);

                tile_entity
            },
        };
        let mut props_item = self.tile_query.get_mut(&mut self.world, tile_entity)
            .map_err(|query_error| EditorError::BadTileEntity {
                tile_pos,
                tile_entity,
                tilemap_entity: self.tilemap_entity,
                query_error,
            })?;
        let old_tile_texture = *props_item.texture;
        let new_tile_texture = props.texture;

        *props_item.color = props.color;
        *props_item.flip = props.flip;
        *props_item.texture = props.texture;

        let mut tile_entity_mut = self.world.entity_mut(tile_entity);
        let Some(tile_data) = &mut self.tile_data else { return Ok(()) };

        if old_tile_texture.0 != new_tile_texture.0 {
            if let Some(old_data) = tile_data.get(&old_tile_texture.0) {
                old_data.remove(&mut tile_entity_mut);
            }
        }

        if let Some(new_data) = tile_data.get(&new_tile_texture.0) {
            new_data.insert(&mut tile_entity_mut);
        }

        Ok(())
    }

    pub fn get_tile_properties(
        &self,
        tile_pos: TilePos,
    ) -> Result<Option<(Entity, TileProperties)>> {
        let Some(tile_entity) = self.get_tile(tile_pos)? else {
            return Ok(None)
        };
        let props_item = self.tile_query.get_manual(&self.world, tile_entity)
            .map_err(|query_error| EditorError::BadTileEntity {
                tile_pos,
                tile_entity,
                tilemap_entity: self.tilemap_entity,
                query_error,
            })?;

        Ok(Some((
            tile_entity,
            TileProperties {
                color: *props_item.color,
                flip: *props_item.flip,
                texture: *props_item.texture,
            }
        )))
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
    ) -> Result<()> {
        painter.add(self.brush_mesh(
            ui_rect,
            self.tile_info(self.brush_state.texture.0)?,
        ));

        Ok(())
    }

    fn tile_info(
        &self,
        id: u32,
    ) -> Result<egui::Rect> {
        let tilemap = self.tilemap_query.get_manual(&self.world, self.tilemap_entity)
            .map_err(|query_error| EditorError::BadTilemapEntity {
                tilemap_entity: self.tilemap_entity,
                query_error,
            })?;

        match &tilemap.texture {
            TilemapTexture::Single(x) => {
                let tile_size = bevy_to_egui(tilemap.tile_size.into());
                let atlas_size = self.world.resource::<Assets<Image>>().get(x)
                    .ok_or(EditorError::InvalidImageHandle {
                        handle: x.clone_weak(),
                    })?
                    .size();
                let pos = tile_id_to_pos(id, bevy_to_egui(atlas_size), tile_size);
                let uv = egui::Rect::from_min_size(pos, tile_size);

                Ok(egui::Rect {
                    min: egui::pos2(uv.min.x / atlas_size.x, uv.min.y / atlas_size.y),
                    max: egui::pos2(uv.max.x / atlas_size.x, uv.max.y / atlas_size.y),
                })
            },
            TilemapTexture::Vector(_) => Err(EditorError::UnsupportedTilemapTextureType("Vector")),
            TilemapTexture::TextureContainer(_) => Err(EditorError::UnsupportedTilemapTextureType("TextureContainer")),
        }
    }

    pub fn copy_tile_properties(
        &mut self,
        tile_pos: TilePos,
    ) -> Result<()> {
        let Some((tile_entity, props)) = self.get_tile_properties(tile_pos)? else { return Ok(()) };
        let tile_entity = self.world.entity(tile_entity);

        *self.brush_state = props;
        let Some(tile_data) = &mut self.tile_data else { return Ok(()) };
        let Some(tile_data) = tile_data.get_mut(&props.texture.0) else { return Ok(()) };

        tile_data.apply(tile_entity);

        Ok(())
    }
}

pub trait Tool: std::fmt::Debug + Send + Sync {
    fn name(&self) -> &'static str;

    fn viewport_ui(
        &mut self,
        ctx: &mut ToolContext,
        hovered_tile: TilePos,
        ui: &mut egui::Ui,
        painter: &Painter,
    ) -> Result<()>;
}
