use std::any::TypeId;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use bevy::ecs::world::EntityMut;
use bevy::prelude::*;
use bevy::reflect::Typed;
use bevy_ecs_tilemap::prelude::*;

use crate::EditorError;

#[derive(Default)]
pub(crate) struct TileData {
    pub(crate) components: HashMap<TypeId, (ReflectComponent, Box<dyn Reflect>)>,
}

#[derive(Default)]
pub(crate) struct EditorTileDataInternal {
    pub (crate) map: HashMap<TilemapTexture, HashMap<u32, TileData>>,
}

impl EditorTileDataInternal {
    pub fn insert(
        &self,
        tileset_info: &TilemapTexture,
        tile_id: &TileTextureIndex,
        entity: &mut EntityMut,
    ) {
        let data = self.map.get(tileset_info)
            .and_then(|x| x.get(&tile_id.0));

        data.into_iter().flat_map(|x| x.components.values())
            .for_each(|(refl, component)| {
                refl.insert(entity, component.as_ref())
            })
    }

    pub fn remove(
        &self,
        tileset_info: &TilemapTexture,
        tile_id: &TileTextureIndex,
        entity: &mut EntityMut,
    ) {
        let data = self.map.get(tileset_info)
            .and_then(|x| x.get(&tile_id.0));


        data.into_iter().flat_map(|x| x.components.values())
            .for_each(|(refl, _)| {
                refl.remove(entity)
            })
    }
}

#[derive(Default, Resource)]
pub struct EditorTileDataRegistry {
    pub(crate) inner: Arc<Mutex<EditorTileDataInternal>>,
}

impl EditorTileDataRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_component<T: Reflect + Typed>(
        &mut self,
        registry: &AppTypeRegistry,
        tileset_info: TilemapTexture,
        tile_id: TileTextureIndex,
        data: T,
    ) -> Result<&mut Self, EditorError> {
        let mut lock = self.inner.lock().unwrap();
        let tileset_registry = lock.map.entry(tileset_info).or_default();
        let tile_registry = tileset_registry.entry(tile_id.0).or_default();

        let registry_lock = registry.read();
        let ty_registration = registry_lock.get(data.type_id())
            .ok_or(EditorError::TypeNotRegistered {
                ty_name: data.type_name().to_string(),
            })?;
        let reflect_component = ty_registration.data::<ReflectComponent>()
            .ok_or(EditorError::TypeNotReflectComponent {
                ty_name: data.type_name().to_string(),
            })?
            .to_owned();

        tile_registry.components.insert(
            ty_registration.type_id(),
            (reflect_component, Box::new(data))
        );

        std::mem::drop(lock);

        Ok(self)
    }
}