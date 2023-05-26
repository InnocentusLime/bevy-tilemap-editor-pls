use bevy::prelude::*;
use bevy_editor_pls::{
    editor_window::{EditorWindow, EditorWindowContext},
    egui, AddEditorWindow,
};

mod coord_utils;
mod error;
mod queries;
mod state;
mod tile_data;

pub use error::EditorError;
pub use state::EditorState;
pub use tile_data::EditorTileDataRegistry;

pub struct TilemapEditorWindow;

impl EditorWindow for TilemapEditorWindow {
    type State = EditorState;
    const NAME: &'static str = "Tilemap editor";

    fn ui(world: &mut World, mut cx: EditorWindowContext, ui: &mut egui::Ui) {
        cx.state_mut::<Self>()
            .expect("Failed to acquire own state")
            .ui(world, ui)
    }

    fn viewport_ui(world: &mut World, mut cx: EditorWindowContext, ui: &mut egui::Ui) {
        cx.state_mut::<Self>()
            .expect("Failed to acquire own state")
            .viewport_ui(world, ui)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct TilemapEditorPlugin;

impl Plugin for TilemapEditorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EditorTileDataRegistry>()
            .add_editor_window::<TilemapEditorWindow>();
    }
}
