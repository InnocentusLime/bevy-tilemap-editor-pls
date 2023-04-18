use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::*;
use bevy_editor_pls::{editor_window::{EditorWindow, EditorWindowContext}, egui, AddEditorWindow};

#[derive(Default, Clone, Copy, Debug)]
pub enum TilemapEditorState {
    #[default]
    PickingTilemap,
    EditingTilemap {
        tilemap: Entity,
        current_tile: TileTextureIndex,
        mirror_flags: TileFlip,
        color: TileColor,
        cursor_pos: TilePos,
    },
}

pub struct TilemapEditorWindow {
    state: TilemapEditorState,
}

impl EditorWindow for TilemapEditorWindow {
    type State = TilemapEditorState;
    const NAME: &'static str = "Tilemap editor";

    fn ui(world: &mut World, cx: EditorWindowContext, ui: &mut egui::Ui) {

    }

    fn viewport_ui(world: &mut World, cx: EditorWindowContext, ui: &mut egui::Ui) {
        let painter = ui.painter_at(ui.max_rect());
        let pos = ui.input(|x| x.pointer.hover_pos());
        if let Some(p) = pos {
            painter.circle_filled(
                p,
                20.0f32,
                egui::Color32::RED,
            );
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct TilemapEditorPlugin;

impl Plugin for TilemapEditorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_editor_window::<TilemapEditorWindow>();
    }
}