use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;
use bevy_editor_pls::egui::Ui;
use bevy::ecs::query::QueryEntityError;
use thiserror::Error;

use crate::queries::EditorQueryStorage;

mod editing_tilemap;
mod picking_tilemap;

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
}

enum Message {
    None,
    ExitTilemapEditing,
    EditTilemap(Entity),
    ShowErrorAndExitEditing(EditorError),
}

struct SharedStateData {
    query_storage: EditorQueryStorage,
}

enum State {
    Editing(editing_tilemap::StateData),
    PickingTilemap(picking_tilemap::StateData),
}

pub struct EditorState {
    state: State,
    shared: SharedStateData,
}

impl EditorState {
    fn state_switch(
        &mut self,
        state: State,
        world: &mut World,
    ) {
        let old = std::mem::replace(&mut self.state, state);

        match old {
            State::Editing(x) => x.cleanup(world),
            State::PickingTilemap(x) => x.cleanup(world),
        }
    }

    fn handle_message(&mut self, msg: Message, world: &mut World) {
        match msg {
            Message::None => (),
            Message::ExitTilemapEditing => {
                let state = picking_tilemap::StateData::new(world, &mut self.shared);

                self.state_switch(State::PickingTilemap(state), world)
            },
            Message::EditTilemap(e) => {
                match editing_tilemap::StateData::new(e, world, &mut self.shared) {
                    Ok(state) =>  self.state_switch(State::Editing(state), world),
                    Err(e) => error!("Error: {e}"),
                }
            },
            // TODO show in the ui
            Message::ShowErrorAndExitEditing(err) => {
                let state = picking_tilemap::StateData::new(world, &mut self.shared);

                error!("The editor has closed due to the following error: {err}");
                self.state_switch(State::PickingTilemap(state), world)
            },
        }
    }

    pub fn ui(&mut self, world: &mut World, ui: &mut Ui) {
        let msg = match &mut self.state {
            State::Editing(x) => x.ui(&mut self.shared, world, ui),
            State::PickingTilemap(x) => x.ui(&mut self.shared, world, ui),
        };

        self.handle_message(msg, world);
    }

    pub fn viewport_ui(&mut self, world: &mut World, ui: &mut Ui) {
        let msg = match &mut self.state {
            State::Editing(x) => x.viewport_ui(&mut self.shared, world, ui),
            State::PickingTilemap(x) => x.viewport_ui(&mut self.shared, world, ui),
        };

        self.handle_message(msg, world);
    }
}

impl Default for EditorState {
    fn default() -> Self {
        Self {
            shared: SharedStateData {
                query_storage: EditorQueryStorage::new(),
            },
            state: State::PickingTilemap(picking_tilemap::StateData::empty())
        }
    }
}