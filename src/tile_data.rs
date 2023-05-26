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

impl TileData {
    pub fn insert(
        &self,
        entity: &mut EntityMut,
    ) {
        self.components.values()
            .for_each(|(refl, component)| {
                refl.insert(entity, component.as_ref())
            })
    }

    pub fn remove(
        &self,
        entity: &mut EntityMut,
    ) {
        self.components.values()
            .for_each(|(refl, _)| {
                refl.remove(entity)
            })
    }
}

#[derive(Default)]
pub(crate) struct EditorTileDataInternal {
    pub (crate) map: HashMap<TilemapTexture, HashMap<u32, TileData>>,
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
        let (type_id, reflect_component) = Self::get_type_data::<T>(registry)?;

        self.insert_component_data(
            tileset_info,
            tile_id,
            data,
            reflect_component,
            type_id
        );

        Ok(self)
    }

    fn insert_component_data<T: Reflect + Typed>(
        &mut self,
        tileset_info: TilemapTexture,
        tile_id: TileTextureIndex,
        data: T,
        reflect_component: ReflectComponent,
        type_id: TypeId,
    ) {
        let mut lock = self.inner.lock().unwrap();

        lock.map
            .entry(tileset_info)
            .or_default()
            .entry(tile_id.0)
            .or_default()
            .components
            .insert(
                type_id,
                (reflect_component, Box::new(data))
            );
    }

    fn get_type_data<T: Reflect + Typed>(registry: &AppTypeRegistry) -> Result<(TypeId, ReflectComponent), EditorError> {
        let ty_name = <T as Typed>::type_info().type_name();
        let type_id = TypeId::of::<T>();

        let registry_lock = registry.read();
        let reflect_component = registry_lock.get(type_id)
            .ok_or(EditorError::TypeNotRegistered {
                ty_name,
            })?
            .data::<ReflectComponent>()
            .ok_or(EditorError::TypeNotReflectComponent {
                ty_name,
            })?
            .to_owned();

        Ok((type_id, reflect_component))
    }
}