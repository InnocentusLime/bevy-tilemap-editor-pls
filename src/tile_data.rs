use std::any::TypeId;
use std::sync::{Arc, Mutex, MutexGuard};
use std::collections::HashMap;

use bevy::ecs::world::{EntityMut, EntityRef};
use bevy::prelude::*;
use bevy::reflect::Typed;
use bevy_ecs_tilemap::prelude::*;

use crate::EditorError;

#[derive(Default)]
pub(crate) struct TileData {
    components: HashMap<TypeId, (ReflectComponent, Box<dyn Reflect>)>,
}

impl TileData {
    pub fn apply(
        &mut self,
        src: EntityRef,
    ) {
        self.components.values_mut()
            .filter_map(|(refl, value)|
                refl.reflect(src)
                .map(|new_value| (value, new_value))
            )
            .for_each(|(value, new_value)| value.apply(new_value))

    }

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

    pub fn values_mut(&'_ mut self) -> impl Iterator<Item = &'_ mut dyn Reflect> {
        self.components.values_mut().map(|x| x.1.as_mut())
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

    pub fn lock(&self) -> EditorTileDataRegistryLock {
        EditorTileDataRegistryLock(self.inner.lock().unwrap())
    }
}

pub struct EditorTileDataRegistryLock<'a>(MutexGuard<'a, EditorTileDataInternal>);

impl<'a> EditorTileDataRegistryLock<'a> {
    pub fn edit_tile_data<'b>(
        &'b mut self,
        registry: &'b AppTypeRegistry,
        tileset_info: TilemapTexture,
        tile_id: TileTextureIndex,
    ) -> TileDataAccess<'b> {
        TileDataAccess(
            self.0.map
                .entry(tileset_info)
                .or_default()
                .entry(tile_id.0)
                .or_default(),
            registry
        )
    }
}

pub struct TileDataAccess<'a>(
    &'a mut TileData,
    &'a AppTypeRegistry,
);

impl<'a> TileDataAccess<'a> {
    pub fn remove<T: Reflect + Typed>(
        &mut self
    ) -> &mut Self {
        self.0.components.remove(&TypeId::of::<T>());

        self
    }

    pub fn insert<T: Reflect + Typed>(
        &mut self,
        data: T,
    ) -> Result<&mut Self, EditorError> {
        let (type_id, reflect_component) = Self::get_type_data::<T>(self.1)?;

        self.0.components.insert(
            type_id,
            (reflect_component, Box::new(data))
        );

        Ok(self)
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