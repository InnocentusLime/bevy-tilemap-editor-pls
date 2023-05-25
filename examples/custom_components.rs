use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_editor_pls::{EditorPlugin, default_windows::cameras::{camera_2d_panzoom::PanCamControls, EditorCamera}};
use bevy_tilemap_editor_pls::{TilemapEditorPlugin, EditorTileDataRegistry};


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
    mut controls: Query<&mut PanCamControls, With<EditorCamera>>,
) {
    controls.single_mut().grab_buttons = vec![MouseButton::Middle];

    let texture_handle: Handle<Image> = asset_server.load("tiles.png");

    let map_size = TilemapSize { x: 32, y: 32 };

    // Setup custom data
    let mut registry = EditorTileDataRegistry::new();
    registry.add_component(
        &app_registry,
        TilemapTexture::Single(texture_handle.cast_weak()),
        TileTextureIndex(0),
        GrassHeight(10),
    ).unwrap();
    registry.add_component(
        &app_registry,
        TilemapTexture::Single(texture_handle.cast_weak()),
        TileTextureIndex(0),
        GroundTag,
    ).unwrap();
    registry.add_component(
        &app_registry,
        TilemapTexture::Single(texture_handle.cast_weak()),
        TileTextureIndex(1),
        WaterTag,
    ).unwrap();
    registry.add_component(
        &app_registry,
        TilemapTexture::Single(texture_handle.cast_weak()),
        TileTextureIndex(2),
        GrassHeight(5),
    ).unwrap();
    registry.add_component(
        &app_registry,
        TilemapTexture::Single(texture_handle.cast_weak()),
        TileTextureIndex(2),
        GroundTag,
    ).unwrap();
    registry.add_component(
        &app_registry,
        TilemapTexture::Single(texture_handle.cast_weak()),
        TileTextureIndex(3),
        GrassHeight(0),
    ).unwrap();
    registry.add_component(
        &app_registry,
        TilemapTexture::Single(texture_handle.cast_weak()),
        TileTextureIndex(3),
        GroundTag,
    ).unwrap();
    registry.add_component(
        &app_registry,
        TilemapTexture::Single(texture_handle.cast_weak()),
        TileTextureIndex(4),
        GroundTag,
    ).unwrap();
    registry.add_component(
        &app_registry,
        TilemapTexture::Single(texture_handle.cast_weak()),
        TileTextureIndex(4),
        HiddenMinerals::Coal,
    ).unwrap();
    registry.add_component(
        &app_registry,
        TilemapTexture::Single(texture_handle.cast_weak()),
        TileTextureIndex(5),
        GroundTag,
    ).unwrap();
    registry.add_component(
        &app_registry,
        TilemapTexture::Single(texture_handle.cast_weak()),
        TileTextureIndex(5),
        HiddenMinerals::Diamonds,
    ).unwrap();
    registry.add_component(
        &app_registry,
        TilemapTexture::Single(texture_handle.cast_weak()),
        TileTextureIndex(5),
        GroundTag,
    ).unwrap();

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.insert_resource(ClearColor(Color::BLACK));
    commands.insert_resource(registry);
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