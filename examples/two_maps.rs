use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_editor_pls::{
    default_windows::cameras::{camera_2d_panzoom::PanCamControls, EditorCamera},
    EditorPlugin,
};
use bevy_tilemap_editor_pls::{EditorTileDataRegistry, TilemapEditorPlugin};

#[derive(Default, Component, Clone, Copy, Reflect)]
#[reflect(Component)]
enum FoodContainer {
    #[default]
    Empty,
    Carrots,
    Corn,
    Cabbage,
    Tomatoes,
    Potatoes,
    Zucchinies,
    Strawberries,
}

#[derive(Default, Component, Clone, Copy, Reflect)]
#[reflect(Component)]
struct WoodAmount(u8);

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    app_registry: Res<AppTypeRegistry>,
    editor_registry: Res<EditorTileDataRegistry>,
    mut controls: Query<&mut PanCamControls, With<EditorCamera>>,
) {
    controls.single_mut().grab_buttons = vec![MouseButton::Middle];

    let texture_handle: Handle<Image> = asset_server.load("pretty_tiles.png");
    let tileset_info = TilemapTexture::Single(texture_handle.cast_weak());

    // Wood tiles
    editor_registry
        .lock()
        .edit_tile_data(&app_registry, tileset_info.clone(), TileTextureIndex(164))
        .insert(WoodAmount(9))
        .unwrap();
    editor_registry
        .lock()
        .edit_tile_data(&app_registry, tileset_info.clone(), TileTextureIndex(165))
        .insert(WoodAmount(3))
        .unwrap();
    editor_registry
        .lock()
        .edit_tile_data(&app_registry, tileset_info.clone(), TileTextureIndex(166))
        .insert(WoodAmount(1))
        .unwrap();

    // Box of cabbage
    editor_registry
        .lock()
        .edit_tile_data(&app_registry, tileset_info.clone(), TileTextureIndex(124))
        .insert(FoodContainer::Cabbage)
        .unwrap();
    editor_registry
        .lock()
        .edit_tile_data(&app_registry, tileset_info.clone(), TileTextureIndex(140))
        .insert(FoodContainer::Cabbage)
        .unwrap();

    // Box of zucchinies
    editor_registry
        .lock()
        .edit_tile_data(&app_registry, tileset_info.clone(), TileTextureIndex(125))
        .insert(FoodContainer::Zucchinies)
        .unwrap();
    editor_registry
        .lock()
        .edit_tile_data(&app_registry, tileset_info.clone(), TileTextureIndex(141))
        .insert(FoodContainer::Zucchinies)
        .unwrap();

    // Box of potatoes
    editor_registry
        .lock()
        .edit_tile_data(&app_registry, tileset_info.clone(), TileTextureIndex(125))
        .insert(FoodContainer::Potatoes)
        .unwrap();
    editor_registry
        .lock()
        .edit_tile_data(&app_registry, tileset_info.clone(), TileTextureIndex(142))
        .insert(FoodContainer::Potatoes)
        .unwrap();

    // Box of tomatoes
    editor_registry
        .lock()
        .edit_tile_data(&app_registry, tileset_info.clone(), TileTextureIndex(126))
        .insert(FoodContainer::Tomatoes)
        .unwrap();
    editor_registry
        .lock()
        .edit_tile_data(&app_registry, tileset_info.clone(), TileTextureIndex(143))
        .insert(FoodContainer::Tomatoes)
        .unwrap();

    // Box of strawberries
    editor_registry
        .lock()
        .edit_tile_data(&app_registry, tileset_info.clone(), TileTextureIndex(156))
        .insert(FoodContainer::Strawberries)
        .unwrap();
    editor_registry
        .lock()
        .edit_tile_data(&app_registry, tileset_info.clone(), TileTextureIndex(172))
        .insert(FoodContainer::Strawberries)
        .unwrap();

    // Box of strawberries
    editor_registry
        .lock()
        .edit_tile_data(&app_registry, tileset_info.clone(), TileTextureIndex(157))
        .insert(FoodContainer::Carrots)
        .unwrap();
    editor_registry
        .lock()
        .edit_tile_data(&app_registry, tileset_info.clone(), TileTextureIndex(173))
        .insert(FoodContainer::Carrots)
        .unwrap();

    // Box of nothing :)
    editor_registry
        .lock()
        .edit_tile_data(&app_registry, tileset_info.clone(), TileTextureIndex(158))
        .insert(FoodContainer::Empty)
        .unwrap();
    editor_registry
        .lock()
        .edit_tile_data(&app_registry, tileset_info.clone(), TileTextureIndex(174))
        .insert(FoodContainer::Empty)
        .unwrap();

    // Box of nothing corn
    editor_registry
        .lock()
        .edit_tile_data(&app_registry, tileset_info.clone(), TileTextureIndex(159))
        .insert(FoodContainer::Corn)
        .unwrap();
    editor_registry
        .lock()
        .edit_tile_data(&app_registry, tileset_info.clone(), TileTextureIndex(175))
        .insert(FoodContainer::Corn)
        .unwrap();

    let map_size = TilemapSize { x: 64, y: 64 };

    let tile_size = TilemapTileSize { x: 32.0, y: 32.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.insert_resource(ClearColor(Color::BLACK));
    commands.spawn((
        TilemapBundle {
            grid_size,
            map_type,
            size: map_size,
            storage: TileStorage::empty(map_size),
            texture: TilemapTexture::Single(texture_handle.clone()),
            tile_size,
            transform: Transform::default().with_translation(Vec3::new(0.0, 0.0, 1.0)),
            ..Default::default()
        },
        Name::new("Background layer"),
    ));
    commands.spawn((
        TilemapBundle {
            grid_size,
            map_type,
            size: map_size,
            storage: TileStorage::empty(map_size),
            texture: TilemapTexture::Single(texture_handle.clone()),
            tile_size,
            transform: Transform::default().with_translation(Vec3::new(0.0, 0.0, 2.0)),
            ..Default::default()
        },
        Name::new("Foreground layer"),
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(TilemapPlugin)
        .add_plugin(EditorPlugin::default())
        .add_plugin(TilemapEditorPlugin)
        .register_type::<FoodContainer>()
        .register_type::<WoodAmount>()
        .add_startup_system(startup)
        .run()
}
