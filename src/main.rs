use bevy::prelude::*;
use bevy_flair::FlairPlugin;

mod assets;
mod board;

use crate::board::{Board, BoardPlugin};

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
    .add_plugins(BoardPlugin)
    .add_systems(Startup, setup);

    app.run();
}


fn setup(mut commands: Commands, board: Res<Board>){
    let board_size = board.get_size();
    let translation = vec3(board_size.0 as f32, board_size.1 as f32, 0.) * (32./2.);
    commands.spawn((Camera2d, Transform::from_translation(translation)));
}
