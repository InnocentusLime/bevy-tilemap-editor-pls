use bevy::prelude::*;
use bevy_editor_pls::egui::Ui;

mod editing_tilemap;
mod picking_tilemap;

enum Message {
    None,
    ExitTilemapEditing,
    EditTilemap(Entity),
}

struct SharedStateData {

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
            Message::ExitTilemapEditing => self.state_switch(State::PickingTilemap(
                picking_tilemap::StateData::new(world)
            ), world),
            Message::EditTilemap(e) => self.state_switch(State::Editing(
                editing_tilemap::StateData::new(e, world)
            ), world),
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
            shared: SharedStateData {  },
            state: State::PickingTilemap(picking_tilemap::StateData::empty())
        }
    }
}