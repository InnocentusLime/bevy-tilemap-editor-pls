use super::*;

use bevy_editor_pls::egui;
use bevy_ecs_tilemap::prelude::*;

#[derive(Debug)]
pub struct TilePicker;

impl Tool for TilePicker {
    fn name(&self) -> &'static str {
        "Picker"
    }

    fn viewport_ui(
        &mut self,
        ctx: &mut ToolContext,
        hovered_tile: TilePos,
        ui: &mut egui::Ui,
        painter: &Painter,
    ) {
        let display_rect = ctx.tile_rect(hovered_tile);
        painter.rect_stroke(
            display_rect,
            0.0,
            egui::Stroke::new(1.0, egui::Color32::RED)
        );

        if ui.input(|x| x.pointer.button_down(egui::PointerButton::Primary)) {
            if let Some(props) = ctx.get_tile_properties(hovered_tile) {
                *ctx.brush_state = props;
            }
        }
    }
}