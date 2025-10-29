use bevy::prelude::*;
use bevy_flair::FlairPlugin;

mod animations;
mod assets;
mod board;
mod ui;
mod menus;

use crate::{board::{Board, BoardPlugin, ShowBoard}, menus::MenusPlugin};

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum GameState {
    #[default]
    Menus,
    InGame,
    InEditor,
}

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Advance Bevy Wars".into(),
                ..default()
            }),
            ..default()
        }),
        FlairPlugin,
    ))
    .init_state::<GameState>()
    .add_computed_state::<ShowBoard>()
    .add_plugins((
        BoardPlugin,
        crate::ui::UiPlugin,
        crate::animations::AnimationPlugin,
        MenusPlugin,
    ))
    .add_systems(Startup, setup);

    app.run();
}

fn setup(mut commands: Commands, board: Res<Board>) {
    let board_size = board.get_size();
    let translation = vec3(board_size.0 as f32, board_size.1 as f32, 0.) * (32. / 2.);
    commands.spawn((Camera2d, Transform::from_translation(translation)));
}

impl ComputedStates for ShowBoard {
    type SourceStates = Option<GameState>;

    fn compute(sources: Self::SourceStates) -> Option<Self> {
        match sources {
            Some(GameState::InGame) => Some(ShowBoard),
            Some(GameState::InEditor) => Some(ShowBoard),
            _ => None,
        }
    }
}
