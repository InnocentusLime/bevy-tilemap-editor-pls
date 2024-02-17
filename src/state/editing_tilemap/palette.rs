use bevy::prelude::*;
use bevy_editor_pls::egui;

use crate::coord_utils::{gridify_int, int_tile_pos_to_id, tile_id_to_pos};

pub struct TilePalette<'a> {
    selected_tile: &'a mut u32,
    palette_size: egui::Vec2,
    tile_size: egui::Vec2,
    palette_texture: egui::TextureId,
}

impl<'a> TilePalette<'a> {
    pub fn new(
        selected_tile: &'a mut u32,
        palette_size: egui::Vec2,
        tile_size: egui::Vec2,
        palette_texture: egui::TextureId,
    ) -> Self {
        Self {
            selected_tile,
            palette_size,
            tile_size,
            palette_texture,
        }
    }

    // Convert coords within palette widget's "world" into global egui coords
    fn local_coords_to_global<R>(
        local_pos: egui::Pos2,
        palette_response: &egui::scroll_area::ScrollAreaOutput<R>,
    ) -> egui::Pos2 {
        local_pos - palette_response.state.offset + palette_response.inner_rect.min.to_vec2()
    }

    // Convert global egui coords into palette's "world" coords
    fn global_coords_to_local<R>(
        global_pos: egui::Pos2,
        palette_response: &egui::scroll_area::ScrollAreaOutput<R>,
    ) -> egui::Pos2 {
        global_pos - palette_response.inner_rect.min.to_vec2() + palette_response.state.offset
    }

    fn palette_size_in_tiles(&self) -> UVec2 {
        UVec2::new(
            (self.palette_size.x / self.tile_size.x) as u32,
            (self.palette_size.y / self.tile_size.y) as u32,
        )
    }

    fn tile_in_bounds(&self) -> bool {
        let pos = tile_id_to_pos(*self.selected_tile, self.palette_size, self.tile_size);

        pos.x < self.palette_size.x && pos.y < self.palette_size.y
    }

    fn paint_tile_picker<R>(
        &self,
        palette_response: &egui::scroll_area::ScrollAreaOutput<R>,
        painter: &egui::Painter,
        tile_id: u32,
    ) {
        let local_pos = tile_id_to_pos(tile_id, self.palette_size, self.tile_size);
        let selected_tile_pos = Self::local_coords_to_global(local_pos, palette_response);

        painter.rect_stroke(
            egui::Rect::from_min_size(selected_tile_pos, self.tile_size),
            0.0,
            egui::Stroke::new(1.0, egui::Color32::RED),
        );
    }
}

impl<'a> egui::Widget for TilePalette<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let palette_response = egui::ScrollArea::both()
            .always_show_scroll(true)
            .max_height(200.0)
            .max_width(200.0)
            .auto_shrink([false; 2])
            .show(ui, |ui| ui.image(self.palette_texture, self.palette_size));
        let painter = ui.painter_at(palette_response.inner_rect);

        // Force the picked tile to zero if its out of range
        if !self.tile_in_bounds() {
            *self.selected_tile = 0;
        }

        // The frame around the selected tile
        self.paint_tile_picker(&palette_response, &painter, *self.selected_tile);

        // The frame around the hovered tile
        let hovered_tile_id = ui
            .input(|x| x.pointer.hover_pos())
            .map(|p| Self::global_coords_to_local(p, &palette_response))
            .map(|p| gridify_int(p, self.tile_size))
            .and_then(|p| int_tile_pos_to_id(p, self.palette_size_in_tiles()))
            .filter(|_| ui.rect_contains_pointer(palette_response.inner_rect));

        if let Some(hovered_tile_id) = hovered_tile_id {
            self.paint_tile_picker(&palette_response, &painter, hovered_tile_id);

            if ui.input(|x| x.pointer.button_clicked(egui::PointerButton::Primary)) {
                *self.selected_tile = hovered_tile_id;
            }
        }

        palette_response.inner
    }
}
