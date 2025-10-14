use bevy::prelude::*;
use bevy_flair::FlairPlugin;

mod assets;
mod board;

use crate::board::BoardPlugin;

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


fn setup(mut commands: Commands){
    commands.spawn((Camera2d, Transform::IDENTITY));
}
