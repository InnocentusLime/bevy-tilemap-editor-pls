use super::*;

use bevy_editor_pls::egui;
use bevy_ecs_tilemap::prelude::*;

#[derive(Debug)]
pub struct TileEraser;

impl Tool for TileEraser {
    fn name(&self) -> &'static str {
        "Eraser"
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
            egui::Stroke::new(1.0, egui::Color32::RED)
        );

        if ui.input(|x| x.pointer.button_down(egui::PointerButton::Primary)) {
            ctx.despawn_tile(hovered_tile)?;
        }

        Ok(())
    }
}