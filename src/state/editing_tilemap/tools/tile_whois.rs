use super::*;

use bevy_editor_pls::egui;
use bevy_ecs_tilemap::prelude::*;

fn flip_flags_to_rotation(
    flip: TileFlip
) -> &'static str {
    // d x y
    let table = [
        /* 0 DXY */ "No transform",
        /* 1 dXY */ "Rotated -90°, then flipped horizontally",
        /* 2 DxY */ "Flipped horizontally",
        /* 3 dxY */ "Rotated -90°",
        /* 4 DXy */ "Flipped vertically",
        /* 5 dXy */ "Rotated 90°",
        /* 6 Dxy */ "Rotated 180°",
        /* 7 dxy */ "Rotated 90°, then flipped horizontally",
    ];

    table[
        flip.d as usize |
        (flip.x as usize) << 1 |
        (flip.y as usize) << 2
    ]
}

#[derive(Debug)]
pub struct TileWhoIs;

impl Tool for TileWhoIs {
    fn name(&self) -> &'static str {
        "Whois"
    }

    fn viewport_ui(
        &mut self,
        ctx: &mut ToolContext,
        hovered_tile: TilePos,
        ui: &mut egui::Ui,
    ) {
        let painter = ui.painter();
        let display_rect = ctx.tile_rect(hovered_tile);
        painter.rect_stroke(
            display_rect,
            0.0,
            egui::Stroke::new(1.0, egui::Color32::RED)
        );

        let ent_and_props = ctx.get_tile(hovered_tile)
            .and_then(|entity|
                ctx.get_tile_properties(hovered_tile)
                .map(|props| (entity, props))
            );
        if let Some((ent, props)) = ent_and_props {
            ui.label(format!(
                "Entity ID: {}",
                ent.index(),
            ));
            ui.label(format!(
                "Diagonal flip: {}",
                props.flip.d as u8,
            ));
            ui.label(format!(
                "Horizontal flip: {}",
                props.flip.x as u8,
            ));
            ui.label(format!(
                "Vertical flip: {}",
                props.flip.y as u8,
            ));
            ui.label(format!(
                "Transform:\n {}",
                flip_flags_to_rotation(props.flip),
            ));
        }
    }
}