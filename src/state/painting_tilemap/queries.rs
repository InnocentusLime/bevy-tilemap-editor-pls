use bevy::prelude::*;
use bevy_editor_pls::egui;
use bevy_ecs_tilemap::prelude::*;

use crate::{tile_id_to_pos, bevy_to_egui};

#[derive(bevy::ecs::query::WorldQuery)]
pub struct TilemapQuery {
    texture: &'static TilemapTexture,
    grid_size: &'static TilemapGridSize,
    tile_size: &'static TilemapTileSize,
    size: &'static TilemapSize,
    transform: &'static GlobalTransform,
}

impl<'a> TilemapQueryItem<'a> {
    // Current merely returns a uv rect, but this will be
    // different for different texture types.
    pub fn picked_tile_info(
        &self,
        id: u32,
        world: &World,
    ) -> egui::Rect {
        match &self.texture {
            TilemapTexture::Single(x) => {
                let tile_size = bevy_to_egui(self.tile_size.into());
                let atlas_size = world.resource::<Assets<Image>>().get(x)
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

#[derive(Debug)]
pub struct TilemapPoints {
    viewport_rect: egui::Rect,
    map_lower_left: Vec2,
    map_top_right: Vec2,
    grid_sample_top_right: Vec2,
}

impl TilemapPoints {
    pub fn tilemap_rect(&self) -> egui::Rect {
        let p1 = self.bevy_viewport_to_egui(self.map_lower_left);
        let p2 = self.bevy_viewport_to_egui(self.map_top_right);

        egui::Rect {
            min: egui::pos2(p1.x, p2.y),
            max: egui::pos2(p2.x, p1.y),
        }.translate(self.viewport_rect.min.to_vec2())
    }

    pub fn grid_sample_rect(&self) -> egui::Rect {
        let p1 = self.bevy_viewport_to_egui(self.map_lower_left);
        let p2 = self.bevy_viewport_to_egui(self.grid_sample_top_right);

        egui::Rect {
            min: egui::pos2(p1.x, p2.y),
            max: egui::pos2(p2.x, p1.y),
        }.translate(self.viewport_rect.min.to_vec2())
    }

    fn bevy_viewport_to_egui(&self, v: Vec2) -> egui::Pos2 {
        egui::pos2(v.x, self.viewport_rect.size().y - v.y)
    }
}

#[derive(bevy::ecs::query::WorldQuery)]
pub struct TilemapCameraQuery {
    camera: &'static Camera,
    transform: &'static Transform,
}

impl<'a> TilemapCameraQueryItem<'a> {
    pub fn is_active(&self) -> bool {
        self.camera.is_active
    }

    // FIXME this 100% doesn't work if the tilemap itself is also transformed.
    pub fn tilemap_points(
        &self,
        viewport_rect: egui::Rect,
        tilemap: &TilemapQueryItem,
    ) -> TilemapPoints {
        let reftile_origin_off = Vec2::from(tilemap.tile_size) / 2.0f32;
        let map_lower_left = tilemap.transform.translation().truncate() - reftile_origin_off;
        let grid_sample_top_right = map_lower_left + Vec2::from(tilemap.grid_size);
        let map_top_right = map_lower_left + Vec2::new(
            (tilemap.size.x as f32) * tilemap.grid_size.x,
            (tilemap.size.y as f32) * tilemap.grid_size.y,
        );

        TilemapPoints {
            viewport_rect,
            map_lower_left: self.world_to_viewport(map_lower_left),
            map_top_right: self.world_to_viewport(map_top_right),
            grid_sample_top_right: self.world_to_viewport(grid_sample_top_right),
        }
    }

    fn world_to_viewport(&self, pos: Vec2) -> Vec2 {
        self.camera.world_to_viewport(
            &(GlobalTransform::IDENTITY * *self.transform),
            pos.extend(1.0),
        ).expect("Transforming failed")
    }
}