use bevy::prelude::*;

use crate::{animations::{AnimationIndices, AnimationTimer}, assets::FileAssets, board::Board};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_cursor)
        .add_systems(Update, follow_cursor);
    }
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
            // bevy::log::info!("Position: {:?}", position);
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
    let animation_indices = AnimationIndices::new( 0,  8 );

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
