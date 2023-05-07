use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_editor_pls::{EditorPlugin, default_windows::cameras::{camera_2d_panzoom::PanCamControls, EditorCamera}};
use bevy_tilemap_editor_pls::{TilemapEditorPlugin, EditorTileDataRegistry};

#[derive(Component, Clone, Copy, Reflect)]
enum FoodContainer {
    Empty,
    Carrots,
    Corn,
    Cabbage,
    Tomatoes,
    Potatoes,
    Zucchinies,
    Strawberries,
}

#[derive(Component, Clone, Copy, Reflect)]
struct WoodAmount(u8);

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut controls: Query<&mut PanCamControls, With<EditorCamera>>,
) {
    controls.single_mut().grab_buttons = vec![MouseButton::Middle];

    let texture_handle: Handle<Image> = asset_server.load("pretty_tiles.png");
    let mut registry = EditorTileDataRegistry::new();
    // Wood tiles
    registry.register(
        TilemapTexture::Single(texture_handle.cast_weak()),
        TileTextureIndex(164),
        WoodAmount(9),
    );
    registry.register(
        TilemapTexture::Single(texture_handle.cast_weak()),
        TileTextureIndex(165),
        WoodAmount(3),
    );
    registry.register(
        TilemapTexture::Single(texture_handle.cast_weak()),
        TileTextureIndex(166),
        WoodAmount(1),
    );
    registry.register(
        TilemapTexture::Single(texture_handle.cast_weak()),
        TileTextureIndex(166),
        WoodAmount(1),
    );
    // Box of cabbage
    registry.register(
        TilemapTexture::Single(texture_handle.cast_weak()),
        TileTextureIndex(124),
        FoodContainer::Cabbage,
    );
    registry.register(
        TilemapTexture::Single(texture_handle.cast_weak()),
        TileTextureIndex(140),
        FoodContainer::Cabbage,
    );
    // Box of zucchinies
    registry.register(
        TilemapTexture::Single(texture_handle.cast_weak()),
        TileTextureIndex(125),
        FoodContainer::Zucchinies,
    );
    registry.register(
        TilemapTexture::Single(texture_handle.cast_weak()),
        TileTextureIndex(141),
        FoodContainer::Zucchinies,
    );
    // Box of potatoes
    registry.register(
        TilemapTexture::Single(texture_handle.cast_weak()),
        TileTextureIndex(125),
        FoodContainer::Potatoes,
    );
    registry.register(
        TilemapTexture::Single(texture_handle.cast_weak()),
        TileTextureIndex(142),
        FoodContainer::Potatoes,
    );
    // Box of tomatoes
    registry.register(
        TilemapTexture::Single(texture_handle.cast_weak()),
        TileTextureIndex(126),
        FoodContainer::Tomatoes,
    );
    registry.register(
        TilemapTexture::Single(texture_handle.cast_weak()),
        TileTextureIndex(143),
        FoodContainer::Tomatoes,
    );
    // Box of strawberries
    registry.register(
        TilemapTexture::Single(texture_handle.cast_weak()),
        TileTextureIndex(156),
        FoodContainer::Strawberries,
    );
    registry.register(
        TilemapTexture::Single(texture_handle.cast_weak()),
        TileTextureIndex(172),
        FoodContainer::Strawberries,
    );
    // Box of strawberries
    registry.register(
        TilemapTexture::Single(texture_handle.cast_weak()),
        TileTextureIndex(157),
        FoodContainer::Carrots,
    );
    registry.register(
        TilemapTexture::Single(texture_handle.cast_weak()),
        TileTextureIndex(173),
        FoodContainer::Carrots,
    );
    // Box of nothing :)
    registry.register(
        TilemapTexture::Single(texture_handle.cast_weak()),
        TileTextureIndex(158),
        FoodContainer::Empty,
    );
    registry.register(
        TilemapTexture::Single(texture_handle.cast_weak()),
        TileTextureIndex(174),
        FoodContainer::Empty,
    );
    // Box of nothing :)
    registry.register(
        TilemapTexture::Single(texture_handle.cast_weak()),
        TileTextureIndex(159),
        FoodContainer::Corn,
    );
    registry.register(
        TilemapTexture::Single(texture_handle.cast_weak()),
        TileTextureIndex(175),
        FoodContainer::Corn,
    );

    let map_size = TilemapSize { x: 64, y: 64 };

    let tile_size = TilemapTileSize { x: 32.0, y: 32.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.insert_resource(ClearColor(Color::BLACK));
    commands.insert_resource(registry);
    commands.spawn((TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: TileStorage::empty(map_size),
        texture: TilemapTexture::Single(texture_handle.clone()),
        tile_size,
        transform: Transform::default().with_translation(Vec3::new(0.0, 0.0, 1.0)),
        ..Default::default()
    }, Name::new("Background layer")));
    commands.spawn((TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: TileStorage::empty(map_size),
        texture: TilemapTexture::Single(texture_handle.clone()),
        tile_size,
        transform: Transform::default().with_translation(Vec3::new(0.0, 0.0, 2.0)),
        ..Default::default()
    }, Name::new("Foreground layer")));
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