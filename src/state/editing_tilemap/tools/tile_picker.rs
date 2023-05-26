use super::*;

use bevy_ecs_tilemap::prelude::*;
use bevy_editor_pls::egui;

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
    ) -> Result<()> {
        let display_rect = ctx.tile_rect(hovered_tile);
        painter.rect_stroke(
            display_rect,
            0.0,
            egui::Stroke::new(1.0, egui::Color32::RED),
        );

        if ui.input(|x| x.pointer.button_down(egui::PointerButton::Primary)) {
            ctx.copy_tile_properties(hovered_tile)?;
        }

        Ok(())
    }
}
