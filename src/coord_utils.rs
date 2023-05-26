use bevy::prelude::*;
use bevy_editor_pls::egui;

pub fn gridify_pos(pos: egui::Pos2, grid_dims: egui::Vec2) -> egui::Pos2 {
    egui::pos2(pos.x / grid_dims.x, pos.y / grid_dims.y).floor()
}

// pub fn snap_to_grid(
//     pos: egui::Pos2,
//     grid_dims: egui::Vec2,
// ) -> egui::Pos2 {
//     let pos = gridify_pos(pos, grid_dims);

//     egui::pos2(
//         pos.x * grid_dims.x,
//         pos.y * grid_dims.y,
//     )
// }

pub fn gridify_int(pos: egui::Pos2, grid_dims: egui::Vec2) -> UVec2 {
    let pos = gridify_pos(pos, grid_dims);

    UVec2::new(pos.x as u32, pos.y as u32)
}

pub fn int_tile_pos_to_id(tile: UVec2, size: UVec2) -> Option<u32> {
    if tile.x >= size.x || tile.y >= size.y {
        return None;
    }

    Some(tile.x + tile.y * size.x)
}

pub fn tile_id_to_pos(id: u32, atlas_size: egui::Vec2, tile_size: egui::Vec2) -> egui::Pos2 {
    let tiles_per_line = (atlas_size.x / tile_size.x) as u32;

    egui::pos2(
        (id % tiles_per_line) as f32 * tile_size.x,
        (id / tiles_per_line) as f32 * tile_size.y,
    )
}

pub fn bevy_to_egui(v: Vec2) -> egui::Vec2 {
    egui::vec2(v.x, v.y)
}
