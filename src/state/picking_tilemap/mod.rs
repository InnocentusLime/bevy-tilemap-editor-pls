use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_editor_pls::egui;

use super::{ SharedStateData, Message };

pub(super) struct StateData {

}

impl StateData {
    pub fn empty() -> Self {
        Self {}
    }

    pub fn new(
        _world: &mut World
    ) -> Self {
        StateData {  }
    }

    pub fn cleanup(
        self,
        _world: &mut World,
    ) {}

    pub fn ui(
        &mut self,
        _shared: &mut SharedStateData,
        world: &mut World,
        ui: &mut egui::Ui,
    ) -> Message {
        // TODO make naming more user-friendly
        let pick = world.query_filtered::<(Entity, Option<&Name>), With<TilemapSize>>()
            .iter(world)
            .find(|(_, name)| ui.button(self.name_tilemap(*name)).clicked());

        if let Some((tilemap, _)) = pick {
            return Message::EditTilemap(tilemap)
        }

        Message::None
    }

    pub fn viewport_ui(
        &mut self,
        _shared: &mut SharedStateData,
        _world: &mut World,
        _ui: &mut egui::Ui,
    ) -> Message {
        Message::None
    }

    fn name_tilemap<'a>(&'a mut self, name: Option<&'a Name>) -> &'a str {
        match name {
            None => "Unnamed tilemap",
            Some(x) => x.as_str(),
        }
    }
}