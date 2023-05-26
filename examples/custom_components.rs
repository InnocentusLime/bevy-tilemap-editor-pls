use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_editor_pls::{
    default_windows::cameras::{camera_2d_panzoom::PanCamControls, EditorCamera},
    EditorPlugin,
};
use bevy_tilemap_editor_pls::{EditorTileDataRegistry, TilemapEditorPlugin};

#[derive(Default, Component, Clone, Copy, Reflect)]
#[reflect(Component)]
struct WaterTag;

#[derive(Default, Component, Clone, Copy, Reflect)]
#[reflect(Component)]
struct GrassHeight(u32);

#[derive(Default, Component, Clone, Copy, Reflect)]
#[reflect(Component)]
struct GroundTag;

#[derive(Default, Component, Clone, Copy, Reflect)]
#[reflect(Component)]
enum HiddenMinerals {
    Diamonds,
    #[default]
    Coal,
}

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    app_registry: Res<AppTypeRegistry>,
    editor_registry: Res<EditorTileDataRegistry>,
    mut controls: Query<&mut PanCamControls, With<EditorCamera>>,
) {
    controls.single_mut().grab_buttons = vec![MouseButton::Middle];

    let texture_handle: Handle<Image> = asset_server.load("tiles.png");

    let map_size = TilemapSize { x: 32, y: 32 };

    // Setup custom data
    let tileset_info = TilemapTexture::Single(texture_handle.cast_weak());

    editor_registry
        .lock()
        .edit_tile_data(&app_registry, tileset_info.clone(), TileTextureIndex(0))
        .insert(GrassHeight(10))
        .unwrap()
        .insert(GroundTag)
        .unwrap();

    editor_registry
        .lock()
        .edit_tile_data(&app_registry, tileset_info.clone(), TileTextureIndex(1))
        .insert(WaterTag)
        .unwrap()
        .insert(GrassHeight(5))
        .unwrap();

    editor_registry
        .lock()
        .edit_tile_data(&app_registry, tileset_info.clone(), TileTextureIndex(2))
        .insert(GroundTag)
        .unwrap()
        .insert(GrassHeight(0))
        .unwrap();

    editor_registry
        .lock()
        .edit_tile_data(&app_registry, tileset_info.clone(), TileTextureIndex(3))
        .insert(GroundTag)
        .unwrap();

    editor_registry
        .lock()
        .edit_tile_data(&app_registry, tileset_info.clone(), TileTextureIndex(4))
        .insert(GroundTag)
        .unwrap()
        .insert(HiddenMinerals::Coal)
        .unwrap();

    editor_registry
        .lock()
        .edit_tile_data(&app_registry, tileset_info.clone(), TileTextureIndex(5))
        .insert(GroundTag)
        .unwrap()
        .insert(HiddenMinerals::Diamonds)
        .unwrap();

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.insert_resource(ClearColor(Color::BLACK));
    commands.spawn(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: TileStorage::empty(map_size),
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        ..Default::default()
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(TilemapPlugin)
        .add_plugin(EditorPlugin::default())
        .add_plugin(TilemapEditorPlugin)
        .register_type::<WaterTag>()
        .register_type::<HiddenMinerals>()
        .register_type::<GrassHeight>()
        .register_type::<GroundTag>()
        .add_startup_system(startup)
        .run()
}
