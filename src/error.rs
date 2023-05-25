use bevy::ecs::query::QueryEntityError;
use bevy::prelude::*;
use thiserror::Error;
use bevy_ecs_tilemap::tiles::TilePos;

#[derive(Debug, Error)]
pub enum EditorError {
    #[error("Tilemap texture type {0:?} isn't supported yet")]
    UnsupportedTilemapTextureType(&'static str),
    #[error("Encountered an incorrect image handle: {handle:?}")]
    InvalidImageHandle {
        handle: Handle<Image>,
    },
    #[error("The tilemap entity {tilemap_entity:?} doesn't exist or is missing some important components")]
    BadTilemapEntity {
        tilemap_entity: Entity,
        #[source]
        query_error: QueryEntityError,
    },
    #[error("The tilemap entity {tilemap_entity:?} has tile {tile_entity:?} at {tile_pos:?}, but it either doesn't exist or is some important components")]
    BadTileEntity {
        tilemap_entity: Entity,
        tile_pos: TilePos,
        tile_entity: Entity,
        #[source]
        query_error: QueryEntityError,
    },
    #[error("The type {ty_name:?} doesn't implement `ReflectComponent`")]
    TypeNotReflectComponent {
        ty_name: String,
    },
    #[error("The type {ty_name:?} isn't registered")]
    TypeNotRegistered {
        ty_name: String,
    },
}