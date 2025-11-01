use bevy::prelude::*;
use bevy_flair::prelude::*;

use crate::{GameState, assets::FileAssets, menus::Menus};
use assets_helper::AssetsTrait;
use ui_helpers::prelude::{button_press_system, clean_entities};
use ui_helpers::{prelude::Action, register_menu};

#[derive(Component)]
pub struct MainMenu;

#[derive(Copy, Clone, Message, Debug)]
pub enum MainMenuActions {
    NewGame,
    Editor,
    Exit,
}

register_menu!(
    register_main_menu,
    Menus::MainMenu,
    MainMenu,
    MainMenuActions,
    spawn_main_menu,
    main_menu_actions_handler
);

pub fn spawn_main_menu(mut commands: Commands, assets_server: Res<AssetServer>) {
    commands.spawn((
        Node::default(),
        NodeStyleSheet::new(FileAssets::MenuStyleMenuCss.load(&assets_server)),
        MainMenu,
        children![
            (Text::new("Bevy Advance Wars"), Name::new("title")),
            (
                Node::default(),
                Name::new("vertical_panel"),
                children![
                    (
                        Button,
                        children![Text::new("New Game"),],
                        Action::new(MainMenuActions::NewGame),
                    ),
                    (
                        Button,
                        children![Text::new("Editor"),],
                        Action::new(MainMenuActions::Editor),
                    ),
                    (
                        Button,
                        Action::new(MainMenuActions::Exit),
                        children![Text::new("Exit"),]
                    )
                ]
            )
        ],
    ));
}

pub fn main_menu_actions_handler(
    mut actions: MessageReader<MainMenuActions>,
    mut state: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
) {
    for action in actions.read() {
        match action {
            MainMenuActions::Exit => {
                exit.write(AppExit::Success);
            }
            MainMenuActions::NewGame => {
                state.set(GameState::InGame);
            }
            MainMenuActions::Editor => {
                state.set(GameState::InEditor);
            }
        }
    }
}
