use std::sync::{Arc, Mutex};

use bevy::ecs::world::EntityMut;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_ecs_tilemap::prelude::*;

trait TileData: Send + Sync {
    fn insert(&self, cmds: &mut EntityMut);

    fn remove(&self, cmds: &mut EntityMut);
}

struct BundledTileData<B>(B);

impl<B: Bundle + Clone> TileData for BundledTileData<B> {
    fn insert(&self, ent: &mut EntityMut) {
        ent.insert(self.0.clone());
    }

    fn remove(&self, ent: &mut EntityMut) {
        ent.remove::<B>();
    }
}

#[derive(Default)]
pub(crate) struct EditorTileDataInternal {
    map: HashMap<TilemapTexture, HashMap<u32, Box<dyn TileData>>>,
}

impl EditorTileDataInternal {
    pub fn insert(
        &self,
        tileset_info: &TilemapTexture,
        tile_id: &TileTextureIndex,
        ent: &mut EntityMut,
    ) {
        let data = self.map.get(tileset_info)
            .and_then(|x| x.get(&tile_id.0));

        if let Some(data) = data {
            data.insert(ent);
        }
    }

    pub fn remove(
        &self,
        tileset_info: &TilemapTexture,
        tile_id: &TileTextureIndex,
        ent: &mut EntityMut,
    ) {
        let data = self.map.get(tileset_info)
            .and_then(|x| x.get(&tile_id.0));

        if let Some(data) = data {
            data.remove(ent);
        }
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

    pub fn register<B: Bundle + Clone>(
        &mut self,
        tileset_info: TilemapTexture,
        tile_id: TileTextureIndex,
        dat: B,
    ) {
        self.inner.lock().unwrap().map.entry(tileset_info).or_default()
            .insert(tile_id.0, Box::new(BundledTileData(dat)));
    }
}