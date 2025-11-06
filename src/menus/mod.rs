use bevy::prelude::*;

use crate::{GameState, menus::main_menu::register_main_menu};

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
        app.add_sub_state::<Menus>();
        register_main_menu(app);
    }
}
