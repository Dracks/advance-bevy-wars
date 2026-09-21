use bevy::prelude::*;
use bevy::ui_widgets::{Activate, Button};
use bevy_flair::prelude::*;

use crate::{GameState, assets::FileAssets, menus::Menus};
use assets_helper::AssetsTrait;

pub fn ui_button(text: impl Into<String>) -> impl Scene {
    let text: String = text.into();
    bsn! {
        Button
        Node
        ClassList::new("button")
        Children[Text::new(text)]
        Interaction
    }
}

pub fn spawn_main_menu() -> impl Scene {
    let css = FileAssets::MenuStyleMenuCss.path();
    bsn! {
        Node
        Styled::StyleSheet(css)
        DespawnOnExit<Menus>(Menus::MainMenu)
        Children[
            (
                Text::new("Bevy Advance Wars")
                Name::new("title")
            ),
            (
                Node::default()
                Name::new("vertical_panel")
                Children[
                    (
                        ui_button("New Game")
                        on(go_to_game)
                    ),
                    (

                    ui_button("Editor")
                        on(go_to_editor)
                    ),
                    (

                    ui_button("Exit")
                        on(exit_game)
                    ),
                ]
            )
        ]
    }
}

fn go_to_game(_: On<Activate>, mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::InGame)
}
fn go_to_editor(_: On<Activate>, mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::InEditor)
}
fn exit_game(_: On<Activate>, mut exit: MessageWriter<AppExit>) {
    exit.write(AppExit::Success);
}
