use bevy::prelude::*;

use crate::{GameState, menus::main_menu::spawn_main_menu};

mod main_menu;

#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(GameState = GameState::Menus)]
pub enum Menus {
    #[default]
    MainMenu,
}

pub struct MenusPlugin;

impl Plugin for MenusPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<Menus>()
            .add_systems(OnEnter(Menus::MainMenu), spawn_main_menu.spawn());
    }
}
