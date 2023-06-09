use bevy::{ecs::query::WorldQuery, prelude::*};
use bevy_ecs_tilemap::prelude::*;
use bevy_editor_pls::egui;

#[derive(bevy::ecs::query::WorldQuery)]
#[world_query(mutable)]
pub struct TilemapQuery {
    pub entity: Entity,
    pub name: Option<&'static Name>,
    pub texture: &'static TilemapTexture,
    pub grid_size: &'static TilemapGridSize,
    pub tile_size: &'static TilemapTileSize,
    pub size: &'static TilemapSize,
    pub transform: &'static GlobalTransform,
    pub storage: &'static mut TileStorage,
    pub ty: &'static TilemapType,
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
        }
        .translate(self.viewport_rect.min.to_vec2())
    }

    pub fn grid_sample_rect(&self) -> egui::Rect {
        let p1 = self.bevy_viewport_to_egui(self.map_lower_left);
        let p2 = self.bevy_viewport_to_egui(self.grid_sample_top_right);

        egui::Rect {
            min: egui::pos2(p1.x, p2.y),
            max: egui::pos2(p2.x, p1.y),
        }
        .translate(self.viewport_rect.min.to_vec2())
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
        tilemap: &TilemapQueryReadOnlyItem,
    ) -> TilemapPoints {
        let reftile_origin_off = Vec2::from(tilemap.tile_size) / 2.0f32;
        let map_lower_left = tilemap.transform.translation().truncate() - reftile_origin_off;
        let grid_sample_top_right = map_lower_left + Vec2::from(tilemap.grid_size);
        let map_top_right = map_lower_left
            + Vec2::new(
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
        self.camera
            .world_to_viewport(
                &(GlobalTransform::IDENTITY * *self.transform),
                pos.extend(1.0),
            )
            .expect("Transforming failed")
    }
}

#[derive(bevy::ecs::query::WorldQuery)]
#[world_query(mutable)]
pub struct TilePropertyQuery {
    pub color: &'static mut TileColor,
    pub flip: &'static mut TileFlip,
    pub texture: &'static mut TileTextureIndex,
}

pub struct EditorQueries<'a> {
    pub tile_query: &'a mut QueryState<TilePropertyQuery>,
    pub tilemap_query: &'a mut QueryState<TilemapQuery>,
    pub camera_query: &'a mut QueryState<TilemapCameraQuery>,
}

pub struct EditorQueryStorage {
    tile_query: Option<QueryState<TilePropertyQuery>>,
    tilemap_query: Option<QueryState<TilemapQuery>>,
    camera_query: Option<QueryState<TilemapCameraQuery>>,
}

impl EditorQueryStorage {
    pub fn new() -> Self {
        Self {
            tile_query: None,
            tilemap_query: None,
            camera_query: None,
        }
    }

    fn access_query<'a, Q: WorldQuery>(
        opt: &'a mut Option<QueryState<Q>>,
        world: &mut World,
    ) -> &'a mut QueryState<Q> {
        let r = opt.get_or_insert_with(|| world.query());

        r.update_archetypes(world);

        r
    }

    pub fn queries(&mut self, world: &mut World) -> EditorQueries<'_> {
        EditorQueries {
            tile_query: Self::access_query(&mut self.tile_query, world),
            tilemap_query: Self::access_query(&mut self.tilemap_query, world),
            camera_query: Self::access_query(&mut self.camera_query, world),
        }
    }
}
