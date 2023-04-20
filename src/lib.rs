use bevy::prelude::*;
use bevy_ecs_tilemap::{tiles::*, prelude::TilemapGridSize};
use bevy_editor_pls::{editor_window::{EditorWindow, EditorWindowContext}, egui::{self, remap}, AddEditorWindow, default_windows::cameras::EditorCamera};

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
        let rect = ui.clip_rect();

        let painter = ui.painter_at(rect);
        let pos = ui.input(|x| x.pointer.hover_pos());

        let mut pan_cam_q = world.query_filtered::<(&Camera, &Transform), (With<Camera2d>, With<EditorCamera>)>();
        let mut tilemap_q = world.query_filtered::<&GlobalTransform, With<TilemapGridSize>>();

        if let Some(p) = pos {
            if !rect.contains(p) {
                return;
            }

            let (cam, cam_tf) = pan_cam_q.single(world);
            let tilemap_tf = tilemap_q.single(world);
            let tilemap_pos = tilemap_tf.translation() - Vec3::new(8.0, 8.0, 0.0);
            let tilemap_ref = tilemap_tf.translation() + Vec3::new(8.0, 8.0, 0.0);

            // determine the pos AFTER the controls system has ran
            let pos = match cam.world_to_viewport(&GlobalTransform::IDENTITY.mul_transform(*cam_tf), tilemap_pos) {
                Some(x) => x,
                None => return,
            };
            let refr = match cam.world_to_viewport(&GlobalTransform::IDENTITY.mul_transform(*cam_tf), tilemap_ref) {
                Some(x) => x,
                None => return,
            };

            // No zoom: [383.3 278.0]
            painter.rect_filled(
                egui::Rect {
                    min: egui::pos2(pos.x, rect.size().y - refr.y) + rect.min.to_vec2(),
                    max: egui::pos2(refr.x, rect.size().y - pos.y) + rect.min.to_vec2(),
                },
                0.0,
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