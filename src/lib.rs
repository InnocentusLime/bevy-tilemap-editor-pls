use bevy::{prelude::*, ecs::query::WorldQuery};
use bevy_ecs_tilemap::prelude::*;
use bevy_editor_pls::{editor_window::{EditorWindow, EditorWindowContext}, egui::{self, remap}, AddEditorWindow, default_windows::cameras::EditorCamera};
use bevy_egui::EguiUserTextures;

#[derive(Default, Clone, Copy, Debug)]
pub enum TilemapEditorStateKind {
    #[default]
    PickingTilemap,
    EditingTilemap {
        tilemap_entity: Entity,
        current_tile: TileTextureIndex,
    },
}

#[derive(Default)]
pub struct TilemapEditorState {
    kind: TilemapEditorStateKind,
    mirror_flags: TileFlip,
    color: TileColor,
}

pub struct TilemapEditorWindow;

#[derive(Debug)]
enum CursorInfo {
    OnMap {
        tile_id: TilePos,
        display_rect: egui::Rect,
    },
    OutofBounds,
}

#[derive(Debug)]
struct EditorView {
    cursor_info: CursorInfo,
    tilemap_bounds: egui::Rect,
}

// Calculates the visual state of the editor when its
// in the tile painting state.
// The pointer post must be given relative to the game viewport
// and WILL be returned relative to the game viewport -- not the global coordinates.
fn tilepainting_view(
    viewport_rect: egui::Rect,
    pointer_pos: egui::Pos2,
    camera: &Camera,
    camera_transform: &GlobalTransform,
    tilemap_transform: &GlobalTransform,
    tilemap_size: &TilemapSize,
    tilemap_grid_size: &TilemapGridSize,
    tilemap_tile_size: &TilemapTileSize,
) -> EditorView {
    // The origin of the tilemap is the center of the bottom-left tile.
    // Knowing that, we can calculate the visual size of a cell IN THE TILE GRID.
    // FIXME this 100% doesn't work if the tilemap itself is also transformed.
    let pointer_pos = pointer_pos - viewport_rect.min.to_vec2();
    let reftile_origin_off = Vec2::from(tilemap_tile_size) / 2.0f32;
    let ref_world_point1 = tilemap_transform.translation().truncate() - reftile_origin_off;
    let ref_world_point2 = ref_world_point1 + Vec2::from(tilemap_grid_size);
    let ref_world_point3 = ref_world_point1 + Vec2::new(
        (tilemap_size.x as f32) * tilemap_grid_size.x,
        (tilemap_size.y as f32) * tilemap_grid_size.y,
    );

    // Now we apply the view camera view to see how the user sees the tile and the tilemap.
    // We readd 0.5 as a z coordinate to ensure that `world_to_viewport` doesn't fail.
    // NOTE: Make a PR for bevy to document when these functions fail.
    let refp_1 = camera.world_to_viewport(
        camera_transform,
        ref_world_point1.extend(0.5),
    ).expect("Coordinate transformation shouldn't have failed (point 1)");
    let refp_2 = camera.world_to_viewport(
        camera_transform,
        ref_world_point2.extend(0.5),
    ).expect("Coordinate transformation shouldn't have failed (point 2)");
    let refp_3 = camera.world_to_viewport(
        camera_transform,
        ref_world_point3.extend(0.5),
    ).expect("Coordinate transformation shouldn't have failed (point 3)");

    // Rects which represent the tile and the tilemap in the viewport
    // Viewport coordinates in bevy have a flipped Y axis
    let ui_origin_tile_rect = egui::Rect {
        min: egui::pos2(refp_1.x, viewport_rect.size().y - refp_2.y),
        max: egui::pos2(refp_2.x, viewport_rect.size().y - refp_1.y),
    };
    let ui_tilemap_rect = egui::Rect {
        min: egui::pos2(refp_1.x, viewport_rect.size().y - refp_3.y),
        max: egui::pos2(refp_3.x, viewport_rect.size().y - refp_1.y),
    };
    let pointer_origin_off = pointer_pos - ui_origin_tile_rect.min.to_vec2();
    let hovered_tile_corner = egui::pos2(
        pointer_origin_off.x / ui_origin_tile_rect.size().x,
        pointer_origin_off.y / ui_origin_tile_rect.size().y,
    ).floor();
    let ui_hovered_tile_corner = egui::pos2(
        hovered_tile_corner.x * ui_origin_tile_rect.size().x,
        hovered_tile_corner.y * ui_origin_tile_rect.size().y,
    ) + ui_origin_tile_rect.min.to_vec2();

    let cursor_info = if ui_tilemap_rect.contains(pointer_pos) {
        CursorInfo::OnMap {
            tile_id: TilePos {
                x: hovered_tile_corner.x as u32,
                y: (-hovered_tile_corner.y) as u32,
            },
            display_rect: egui::Rect {
                min: ui_hovered_tile_corner,
                max: ui_hovered_tile_corner + ui_origin_tile_rect.size(),
            }.translate(viewport_rect.min.to_vec2()),
        }
    } else {
        CursorInfo::OutofBounds
    };

    EditorView {
        cursor_info,
        tilemap_bounds: ui_tilemap_rect.translate(viewport_rect.min.to_vec2()),
    }
}

fn tilepainting_state(
    tilemap_entity: Entity,
    current_tile: TileTextureIndex,
    world: &mut World,
    ui: &mut egui::Ui,
) {
    let viewport_rect = ui.clip_rect();
    // FIXME this painter doesn't have the top bar clipped.
    let painter = ui.painter_at(viewport_rect);
    let pos = ui.input(|x| x.pointer.hover_pos());

    let mut pan_cam_q = world.query_filtered::<(&Camera, &Transform), (With<Camera2d>, With<EditorCamera>)>();
    let mut tilemap_q = world.query_filtered::<(&GlobalTransform, &TilemapSize, &TilemapGridSize, &TilemapTileSize, &TileStorage), With<TilemapGridSize>>();
    let pointer_pos = match pos {
        Some(x) if ui.ui_contains_pointer() => x,
        _ => return,
    };

    let (camera, camera_transform) = pan_cam_q.single(world);
    let (
        tilemap_transform,
        tilemap_size,
        tilemap_grid_size,
        tilemap_tile_size,
        tile_storage,
    ) = match tilemap_q.get(world, tilemap_entity) {
        Ok(x) => x,
        Err(_) => panic!("AMOGUS"),
    };
    let view = tilepainting_view(
        viewport_rect,
        pointer_pos,
        camera,
        &(GlobalTransform::IDENTITY * (*camera_transform)),
        tilemap_transform,
        tilemap_size,
        tilemap_grid_size,
        tilemap_tile_size,
    );

    painter.rect_stroke(
        view.tilemap_bounds,
        0.0,
        egui::Stroke::new(4.0, egui::Color32::RED),
    );
    match view.cursor_info {
        CursorInfo::OutofBounds => (),
        CursorInfo::OnMap { tile_id, display_rect } => {
            painter.rect_filled(
                display_rect,
                0.0,
                egui::Color32::RED,
            );

            if ui.input(|x| x.key_down(egui::Key::T)) {
                let tile = tile_storage.get(&tile_id).unwrap();
                let mut tex_index = world.query::<&mut TileTextureIndex>()
                    .get_mut(world, tile)
                    .unwrap();
                *tex_index = current_tile;
            }
        },
    }
}

impl EditorWindow for TilemapEditorWindow {
    type State = TilemapEditorState;
    const NAME: &'static str = "Tilemap editor";

    fn ui(world: &mut World, mut cx: EditorWindowContext, ui: &mut egui::Ui) {
        let state = cx.state_mut::<Self>().unwrap();

        match &mut state.kind {
            TilemapEditorStateKind::PickingTilemap => {
                let pick = world.query_filtered::<Entity, With<TilemapSize>>()
                    .iter_manual(world)
                    .find(|_| ui.button("Tilemap").clicked());

                if let Some(tilemap) = pick {
                    state.kind = TilemapEditorStateKind::EditingTilemap {
                        tilemap_entity: tilemap,
                        current_tile: TileTextureIndex(0),
                    };
                }
            },
            TilemapEditorStateKind::EditingTilemap {
                tilemap_entity,
                current_tile,
            } => {
                if ui.button("Exit").clicked() {
                    state.kind = TilemapEditorStateKind::PickingTilemap;
                    return;
                }

                let texture = world.query::<&TilemapTexture>()
                    .get(world, *tilemap_entity)
                    .unwrap()
                    .clone();
                let (texture_id, texture_size) = match texture {
                    TilemapTexture::Single(x) => {
                        let mut res = world.resource_mut::<EguiUserTextures>();
                        let id = res.add_image(x.clone());
                        let size = world.resource::<Assets<Image>>()
                            .get(&x)
                            .unwrap()
                            .size();
                        (id, size)
                    },
                    _ => todo!(),
                };

                let resp = egui::ScrollArea::both()
                .always_show_scroll(true)
                .max_height(200.0)
                .max_width(200.0)
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    ui.image(
                        texture_id,
                        egui::vec2(texture_size.x, texture_size.y),
                    )
                });

                let rect = resp.inner_rect;
                if !ui.rect_contains_pointer(rect) {
                    return;
                }
                let pos = if let Some(p) = ui.input(|x| x.pointer.hover_pos()) {
                    p
                } else { return; };

                let p = pos - rect.min.to_vec2();
                let world_pos = p + resp.state.offset;
                if world_pos.x >= texture_size.x || world_pos.y >= texture_size.y {
                    return;
                }

                let tile_offset = egui::pos2(
                    world_pos.x / 16.0,
                    world_pos.y / 16.0,
                ).floor();
                let tile_ui_offset = egui::pos2(
                    tile_offset.x * 16.0,
                    tile_offset.y * 16.0,
                ) - resp.state.offset + rect.min.to_vec2();

                ui.painter().rect_stroke(
                    egui::Rect::from_min_size(
                        tile_ui_offset,
                        egui::vec2(16.0, 16.0),
                    ),
                    0.0,
                    egui::Stroke::new(1.0, egui::Color32::RED)
                );

                if ui.input(|x| x.key_pressed(egui::Key::P)) {
                    let tile_id = tile_offset.x as u32 +
                        (tile_offset.y as u32) * (texture_size.x as u32) / 16;
                    *current_tile = TileTextureIndex(tile_id);
                }
            }
        }
    }

    fn viewport_ui(world: &mut World, cx: EditorWindowContext, ui: &mut egui::Ui) {
        let state = cx.state::<Self>().unwrap();

        match state.kind {
            TilemapEditorStateKind::PickingTilemap => (),
            TilemapEditorStateKind::EditingTilemap { tilemap_entity, current_tile } => tilepainting_state(
                tilemap_entity,
                current_tile,
                world,
                ui,
            ),
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