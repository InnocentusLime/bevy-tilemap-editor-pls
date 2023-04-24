use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_editor_pls::{egui_dock, egui, default_windows::cameras::EditorCamera};
use bevy_egui::EguiUserTextures;

use crate::{bevy_to_egui, gridify_int};

use super::{ SharedStateData, Message };

mod palette;
mod queries;

use palette::TilePalette;
use queries::{ TilemapCameraQuery, TilemapQuery };

pub(super) struct StateData {
    // editor state stuff
    selected_tile: TileTextureIndex,
    tile_mirror_flags: TileFlip,
    tile_color: TileColor,
    // bevy stuff
    tilemap_texture: Handle<Image>,
    tilemap_entity: Entity,
    // egui stuff
    tilemap_texture_egui: egui::TextureId,
}

impl StateData {
    pub fn new(
        tilemap_entity: Entity,
        world: &mut World,
    ) -> Self {
        // Extract the atlas image and register it
        // FIXME this solution supports only single-image atlases
        let texture = world.query::<&TilemapTexture>()
            .get(world, tilemap_entity)
            .expect("The passes entity doesn't have a texture")
            .clone();
        let (tilemap_texture, tilemap_texture_egui) = match texture {
            TilemapTexture::Single(x) => (
                x.clone(),
                world.resource_mut::<EguiUserTextures>().add_image(x),
            ),
            // FIXME needs careful tweaking due to "atlas feature"
            TilemapTexture::Vector(_) => todo!(),
            TilemapTexture::TextureContainer(_) => todo!(),
        };

        Self {
            // editor state stuff
            selected_tile: TileTextureIndex(0),
            tile_mirror_flags: TileFlip::default(),
            tile_color: TileColor::default(),
            // bevy stuff
            tilemap_texture,
            tilemap_entity,
            // egui stuff
            tilemap_texture_egui,
        }
    }

    pub fn cleanup(
        self,
        world: &mut World,
    ) {
        world.resource_mut::<EguiUserTextures>().remove_image(&self.tilemap_texture);
    }

    pub fn ui(
        &mut self,
        _shared: &mut SharedStateData,
        world: &mut World,
        ui: &mut egui::Ui,
    ) -> Message {
        // Fetch some info about the tilemap and its atlas
        let tile_size: Vec2 = world.query::<&TilemapTileSize>()
            .get(world, self.tilemap_entity)
            .expect("Selected tilemap has no tile size")
            .into();
        let atlas_size = world.resource::<Assets<Image>>()
            .get(&self.tilemap_texture)
            .expect("Invalid texture handle")
            .size();

        if ui.button("Exit").clicked() {
            return Message::ExitTilemapEditing;
        }

        ui.add(TilePalette::new(
            &mut self.selected_tile.0,
            bevy_to_egui(atlas_size),
            bevy_to_egui(tile_size),
            self.tilemap_texture_egui,
        ));

        ui.separator();

        ui.checkbox(&mut self.tile_mirror_flags.x, "Horizontal flip");
        ui.checkbox(&mut self.tile_mirror_flags.y, "Vertical flip");
        ui.checkbox(&mut self.tile_mirror_flags.d, "Diagonal flip");

        let mut tile_rgba = self.tile_color.0.as_rgba_f32();
        ui.color_edit_button_rgba_unmultiplied(&mut tile_rgba);
        self.tile_color.0 = Color::rgba(tile_rgba[0], tile_rgba[1], tile_rgba[2], tile_rgba[3]);

        // TODO hotkey for rotation
        // TODO make the keys configurable
        if ui.input(|x| x.key_pressed(egui::Key::H)) {
            self.tile_mirror_flags.x = !self.tile_mirror_flags.x;
        }
        if ui.input(|x| x.key_pressed(egui::Key::V)) {
            self.tile_mirror_flags.y = !self.tile_mirror_flags.y;
        }
        if ui.input(|x| x.key_pressed(egui::Key::D)) {
            self.tile_mirror_flags.d = !self.tile_mirror_flags.d;
        }

        Message::None
    }

    pub fn viewport_ui(
        &mut self,
        _shared: &mut SharedStateData,
        world: &mut World,
        ui: &mut egui::Ui,
    ) -> Message {
        let viewport_rect = ui.clip_rect();
        let mut clip_rect = viewport_rect;
        clip_rect.set_top(
            ui.min_rect().top() -
            egui_dock::Style::default().default_inner_margin.top
        );
        let mut painter = ui.painter_at(clip_rect);
        painter.set_layer_id(egui::LayerId::background());

        // Fetch information about the tilemap and the cursor
        let mut cam_q = world.query_filtered::<TilemapCameraQuery, (With<EditorCamera>, With<Camera2d>)>();
        let mut tilemap_q = world.query::<TilemapQuery>();
        let Some(cam) = cam_q.iter(world)
            .find(|x| x.is_active())
        else {
            return Message::None;
        };
        let tilemap = tilemap_q.get(world, self.tilemap_entity)
            .expect("Invalid tilemap");

        // Compute the display rects
        let ref_points = cam.tilemap_points(viewport_rect, &tilemap);
        let grid_sample_rect = ref_points.grid_sample_rect();
        let tilemap_rect = ref_points.tilemap_rect();

        // Paint a frame around the whole tilemap
        painter.rect_stroke(
            tilemap_rect,
            0.0,
            egui::Stroke::new(2.0, egui::Color32::RED),
        );

        // Paint a frame and a tile at the place where the pointer is
        let hovered_tile = ui.input(|x| x.pointer.hover_pos())
            .and_then(|p| Self::global_pos_to_local(p, tilemap_rect))
            .map(|p| gridify_int(p, grid_sample_rect.size()))
            .filter(|_| ui.ui_contains_pointer());
        if let Some(hovered_tile) = hovered_tile {
            let info = tilemap.picked_tile_info(self.selected_tile.0, world);
            self.paint_tile_pointer(
                &painter,
                grid_sample_rect,
                info,
                hovered_tile,
            );

            if ui.input(|x| x.key_pressed(egui::Key::T)) {
                let get_res = world.query::<&TileStorage>()
                    .get_mut(world, self.tilemap_entity)
                    .expect("Tilemap has no storage")
                    .get(&hovered_tile.into());

                match get_res {
                    Some(e) => {
                        let (mut texture_index, mut color, mut flip) =
                            world.query::<(&mut TileTextureIndex, &mut TileColor, &mut TileFlip)>()
                            .get_mut(world, e)
                            .expect("Tile contains no texture component");
                        *texture_index = self.selected_tile;
                        *color = self.tile_color;
                        *flip = self.tile_mirror_flags;
                    },
                    None => todo!("Spawn tile"),
                }
            }
        }

        Message::None
    }

    fn paint_tile_pointer(
        &self,
        painter: &egui::Painter,
        grid_sample_rect: egui::Rect,
        info: egui::Rect,
        tile_pos: UVec2,
    ) {
        let display_rect = Self::selected_tile_rect(tile_pos, grid_sample_rect);

        painter.image(
            self.tilemap_texture_egui,
            display_rect,
            info,
            egui::Color32::WHITE,
        );
        painter.rect_stroke(
            display_rect,
            0.0,
            egui::Stroke::new(1.0, egui::Color32::RED)
        );
    }

    fn selected_tile_rect(
        tile: UVec2,
        sample_rect: egui::Rect,
    ) -> egui::Rect {
        let size = sample_rect.size();

        // Flip the sign because bigger `y` in `TilePos` means "higher"
        // instead of "lower"
        sample_rect.translate(egui::vec2(
            tile.x as f32 * size.x,
            -(tile.y as f32) * size.y,
        ))
    }

    // The y component is computed differently, so the higher you go,
    // the bigger y component of the result gets.
    fn global_pos_to_local(
        pos: egui::Pos2,
        tilemap_rect: egui::Rect,
    ) -> Option<egui::Pos2> {
        let tilemap_size = tilemap_rect.size();
        let pos = egui::pos2(
            pos.x - tilemap_rect.min.x,
            tilemap_rect.max.y - pos.y,
        );

        if pos.x <= 0.0 || pos.y <= 0.0 || pos.x >= tilemap_size.x || pos.y >= tilemap_size.y {
            return None;
        }

        Some(pos)
    }
}