use assets_helper::AssetsTrait;
use auto_tiler::BoardTrait;
use bevy::prelude::*;
use bevy_flair::style::components::NodeStyleSheet;

use crate::{
    animations::{AnimationIndices, AnimationTimer},
    assets::FileAssets,
    board::Board,
};

pub struct UiPlugin;

#[derive(Message)]
pub struct HoverCell {
    cell: UVec2,
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, update_ui)
            .add_systems(Update, follow_cursor)
            .add_message::<HoverCell>();
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
    mut hover: MessageWriter<HoverCell>,
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
            hover.write(HoverCell {
                cell: ivec2(coord_x, coord_y).try_into().unwrap(),
            });
        }
    }
}

#[derive(Component)]
struct TileInfo;

#[derive(Component)]
struct GameUI;

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let cursor_img = FileAssets::ImagesGameCursorHud36X36Png.load(&assets);
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(36), 9, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices::new(0, 8);

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

    commands.spawn((
        NodeStyleSheet::new(FileAssets::MenuStyleUiCss.load(&assets)),
        Text::default(),
        Node::default(),
        Name::new("tile-info"),
        TileInfo,
        GameUI,
        children![
            TextSpan(format!("\n")),
            TextSpan(format!("\n",)),
            TextSpan(format!("\n")),
        ],
    ));
}

fn update_ui(
    tile_info: Single<Entity, With<TileInfo>>,
    mut writer: TextUiWriter,
    mut hover_reader: MessageReader<HoverCell>,
    board: Res<Board>,
) {
    for msg in hover_reader.read() {
        if let Some(terrain) = board.get(&msg.cell) {
            *writer.text(*tile_info, 1) = format!("{:?}\n", terrain);
        }
    }
}
