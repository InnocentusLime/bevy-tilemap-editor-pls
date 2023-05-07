# What is this

This is a simple tilemap editor plugin, that hooks right into `bevy_editor_pls` to work with `bevy_ecs_tilemap`. It works completely within its own confines and doesn't spawn any additional entities. The current aim is to provide a good enough editor to do most of the level editing tasks right inside the bevy application.

To use this plugin just add it to your app:

```rust
    app.add_plugin(TilemapEditorPlugin);
```

Then you should see a `Tile editor` option in the `Open Window` menu in `bevy_editor_pls`.

It is STRONGLY recommended to change the camera panning controls in `bevy_editor_pls` to exclude both left and right mouse buttons, since those are used by the editor. In the examples you can see how to override the camera panning controls to mouse's middle button.

In addition, if you have some additional components, that you attach to the tiles, you can configure a resource that tells the editor what components it should insert

```rust
    let mut registry = EditorTileDataRegistry::default();

    registry.register(
        TilemapTexture::Single(texture_handle),
        TileTextureIndex(0),
        (GrassHeight(10), GroundTag),
    );

    commands.insert_resource(registry);
```

# Why use this?

* As of this writing, both `tiled` and `ldtk` aren't compatible with bevy without some tinkering.
* Debugging any sort of game without any built in tile editor is a complete pain, even with an inspector.
* Neither `tiled` or `ldtk` really reflect the ecs-like approach of `bevy_ecs_tilemap`.

# Known issues / intricacies

- There is a lot of "expect()" and "unwrap()" in the code. Some of them aren't really justified and will for sure crash the app. They should be replaced with returning a `Result<T, E>`.
- Currently, there's not "proper" layer support. `bevy_ecs_tilemap` allows emulating layers by spawning a separate tilemap for that. In other editors you can simply reorder the layers, which is easier-said-than-done when it comes to bevy, since all objects are z-coordinate ordered.
- The plugin doesn't support tilemaps that have gone through `Transforms` other than mere translation.
- Tile painting can spill out when your mouse button is still down.
- This plugin relies on `bevy_editor_pls`, which is known to have some (performance issues)[https://github.com/jakobhellermann/bevy_editor_pls/issues/20].

# Features

## Avaliable right now

- Painting tiles
- Erasing tiles
- Spawning tiles with a bundle
- Overriding the tile components by painting
- Picking the tile
- Discovering tile entity ID by hovering over it

## Will implement in the nearest future

- Support for other atlas formats
- No reliance on `bevy_editor_pls`
- Tilemap saving
- Tilemap loading
- Live tilemap resizing
- Animation editing
- Showing-hiding a whole tilemap in the editor
- Taking tilemap scaling into account
- A thin grid to make it easier to measure space in a tilemap
- Rotating tiles
- Custom tools

## Will implement, but they aren't a high priority right now

- Configurable controls
- Spawning a tilemap
- Taking tilemap rotation into account
- Compatability with prefab crates like `bevy_proto`

## Won't implement

- Editing for tilemaps other than square maps (this project already has a quite big scope. Adding suport for other tilemap types would slow things down)
- Spawning non-tile entities (once again, it is out of the current scope I have set up and it's probably a task that shouldn't be done by this plugin)
- Fixed file format for tilemaps (while this plugin will for sure support loading tilemaps from a file, it definitely won't be locked to any specific tilemap format)

# Credits

## The "pretty_tiles.png" tileset

Credit goes to: Zabin, Daneeklu, Jetrel, Hyptosis, Redshrike, Bertram. Link to the tileset's page: http://opengameart.org/content/rpg-tiles-cobble-stone-paths-town-objects