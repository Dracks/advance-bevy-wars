use bevy::{math::ops::powf, prelude::*};

const GRID_WIDTH: usize = 68;
const GRID_HEIGHT: usize = 45;
const TILE_SIZE: f32 = 32.0;

#[derive(Component)]
struct GridTile {
    x: usize,
    y: usize,
}

#[derive(Component)]
struct CoordinateText;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (follow_cursor, basic_camera))
        .run();
}

#[derive(Component)]
struct Board;

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((Camera2d, Transform::default()));
    commands.spawn((Transform::default(), Visibility::Visible, Board))
        .with_children(|parent|{
            parent.spawn((
                Sprite::from_image(assets.load("images/game/terrain.png")),
                Transform::default(),
            ));

            // Calculate grid offset to center it
            let grid_width_pixels = GRID_WIDTH as f32 * TILE_SIZE;
            let grid_height_pixels = GRID_HEIGHT as f32 * TILE_SIZE;
            let offset_x = -grid_width_pixels / 2.0 + TILE_SIZE / 2.0;
            let offset_y = -grid_height_pixels / 2.0 + TILE_SIZE / 2.0;

            // Create grid tiles
            for y in 0..GRID_HEIGHT {
                for x in 0..GRID_WIDTH {
                    let pos_x = offset_x + x as f32 * TILE_SIZE;
                    let pos_y = offset_y + y as f32 * TILE_SIZE;

                    // Alternating tile colors
                    let color = if (x + y) % 2 == 0 {
                        Color::srgba(0.8, 0.8, 0.8, 0.1)
                    } else {
                        Color::srgba(0.6, 0.6, 0.6, 0.1)
                    };

                    parent.spawn((
                        Mesh2d(meshes
                            .add(Rectangle::new(TILE_SIZE, TILE_SIZE))
                            .into()),
                        MeshMaterial2d(materials.add(color)),
                        Transform::from_xyz(pos_x, pos_y, 0.0),
                        GridTile { x, y: GRID_HEIGHT-y-1 },
                    ));
                }
            }

            // Draw grid lines
            let line_color = Color::srgb(0.2, 0.2, 0.2);

            // Vertical lines
            for x in 0..=GRID_WIDTH {
                let pos_x = offset_x - TILE_SIZE / 2.0 + x as f32 * TILE_SIZE;
                parent.spawn((
                    Mesh2d(meshes.add(Rectangle::new(2.0, grid_height_pixels)).into()),
                    MeshMaterial2d(materials.add(line_color)),
                    Transform::from_xyz(pos_x, 0.0, 1.0),
                ));
            }

            // Horizontal lines
            for y in 0..=GRID_HEIGHT {
                let pos_y = offset_y - TILE_SIZE / 2.0 + y as f32 * TILE_SIZE;
                parent.spawn((
                    Mesh2d(meshes.add(Rectangle::new(grid_width_pixels, 2.0)).into()),
                    MeshMaterial2d(materials.add(line_color)),
                    Transform::from_xyz(0.0, pos_y, 1.0),
                ));
            }
        });

    // Coordinate display text
    commands.spawn((
        Text::new(""),
        children![TextSpan("Hover over a tile".to_string())],
        TextColor::WHITE,
        TextFont::from_font_size(24.0),
        CoordinateText,
        Node{
            top: Val::Px(10.),
            ..Default::default()
        }
    ));
}

fn follow_cursor(
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    tiles: Query<(&GridTile, &GlobalTransform)>,
    text_query: Single<Entity, With<CoordinateText>>,
    mut writer: TextUiWriter,
) {
    let Ok(window) = windows.single() else {
        bevy::log::error!("No window found");
        return;
    };
    let Ok((camera, camera_transform)) = camera_q.single() else {
        bevy::log::error!("No camera found");
        return;
    };

    if let Some(cursor_pos) = window.cursor_position() {
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
            // Find which tile the cursor is over
            for (tile, transform) in tiles.iter() {
                let tile_pos = transform.translation().truncate();
                let half_size = TILE_SIZE / 2.0;

                if world_pos.x >= tile_pos.x - half_size
                    && world_pos.x <= tile_pos.x + half_size
                    && world_pos.y >= tile_pos.y - half_size
                    && world_pos.y <= tile_pos.y + half_size
                {
                    // Update text with coordinates
                    *writer.text(*text_query, 1) = format!("Tile: ({}, {})", tile.x, tile.y);
                    return;
                }
            }
        }
    }

    // If no tile is hovered, show default text
    *writer.text(*text_query, 1) = "Hover over a tile".to_string();
}

fn basic_camera(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut board: Single<&mut Transform, With<Board>>,
    time: Res<Time>,
) {
    let mut transform = Vec2::ZERO;
    let mut input_scale = 0.0;
    if keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
        transform.x = 1.0
    }
    if keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
        transform.x -= 1.0
    }
    if keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]) {
        transform.y = -1.0
    }
    if keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]) {
        transform.y += 1.0
    }

    if keyboard_input.any_pressed([KeyCode::KeyR]){
        input_scale = 1.0
    }
    if keyboard_input.any_pressed([KeyCode::KeyF]){
        input_scale -= 1.0
    }

    transform = transform * time.delta_secs()*100.0;
    let scale = powf(2.0,  input_scale*time.delta_secs());

    board.translation += vec3(transform.x, transform.y, 0.0);
    board.scale = vec3(board.scale.x * scale, board.scale.y*scale, 1.0);
}
