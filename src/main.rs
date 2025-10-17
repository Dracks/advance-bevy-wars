use bevy::prelude::*;
use bevy_flair::FlairPlugin;

mod assets;
mod board;

use crate::{
    assets::FileAssets,
    board::{Board, BoardPlugin, MainBoard},
};

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
    .add_systems(Startup, (setup, setup_cursor))
    .add_systems(Update, (follow_cursor, animate_sprite));

    app.run();
}

#[derive(Component, Default)]
struct Cursor {
    position: UVec2,
}

fn follow_cursor(
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    board: Res<Board>,
    mut game_cursor: Single<&mut Transform, With<Cursor>>,
) {
    let Ok(window) = windows.single() else {
        bevy::log::error!("No window found");
        return;
    };
    let Ok((camera, camera_transform)) = camera_q.single() else {
        bevy::log::error!("No camera found");
        return;
    };

    if let Some(cursor) = window.cursor_position() {
        if let Ok(position) = camera.viewport_to_world_2d(camera_transform, cursor) {
            bevy::log::info!("Position: {:?}", position);
            let coord_x = ((position.x as i32) + 16) / 32;
            let coord_y = ((position.y as i32) + 16) / 32;
            if coord_x < 0 || coord_y < 0 {
                return;
            }
            let board_size = board.get_size();
            if coord_x >= board_size.0 as i32 || coord_y >= board_size.1 as i32 {
                return;
            }
            let new_x = (coord_x as f32) * 32.0;
            let new_y = (coord_y as f32) * 32.0;
            game_cursor.translation = vec3(new_x, new_y, game_cursor.translation.y);
        }
    }
}

fn setup_cursor(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let cursor_img = FileAssets::ImagesGameCursorHud36X36.load(&assets);
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(36), 9, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices { first: 0, last: 8 };

    commands.spawn((
        Cursor::default(),
        Sprite::from_atlas_image(
            cursor_img,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: 1,
            },
        ),
        Transform::from_translation(Vec3::Z * 4.),
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        animation_indices,
    ));
}

fn setup(mut commands: Commands, board: Res<Board>) {
    let board_size = board.get_size();
    let translation = vec3(board_size.0 as f32, board_size.1 as f32, 0.) * (32. / 2.);
    commands.spawn((Camera2d, Transform::from_translation(translation)));
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished()
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}
