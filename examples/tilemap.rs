use bevy::{
    prelude::*,
    render::render_resource::Extent3d,
    sprite_render::{TileData, TilemapChunk, TilemapChunkTileData},
};
use bevy_flair::FlairPlugin;
use rand::Rng;

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
    .add_systems(Startup, setup)
    .add_systems(Update, update_tileset_image);

    app.run();
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    // We're seeding the PRNG here to make this example deterministic for testing purposes.
    // This isn't strictly required in practical use unless you need your app to be deterministic.
    let mut rng = rand::rng();

    let chunk_size = UVec2::splat(64);
    let tile_display_size = UVec2::splat(32);
    let tile_data: Vec<Option<TileData>> = (0..chunk_size.element_product())
        .map(|_| rng.random_range(0..765))
        .map(|i| {
            if i == 0 {
                None
            } else {
                Some(TileData::from_tileset_index(i - 1))
            }
        })
        .collect();

    commands.spawn((
        TilemapChunk {
            chunk_size,
            tile_display_size,
            tileset: assets.load("images/game/terrain.png"),
            ..default()
        },
        TilemapChunkTileData(tile_data),
        // UpdateTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));

    commands.spawn(Camera2d);

    // commands.insert_resource(SeededRng(rng));
}

fn update_tileset_image(
    chunk_query: Single<&TilemapChunk>,
    mut events: MessageReader<AssetEvent<Image>>,
    mut images: ResMut<Assets<Image>>,
) {
    let chunk = *chunk_query;
    for event in events.read() {
        if event.is_loaded_with_dependencies(chunk.tileset.id()) {
            let image = images.get_mut(&chunk.tileset).unwrap();
            image.reinterpret_size(Extent3d {
                width: 64,
                height: 64,
                depth_or_array_layers: 765,
            });
        }
    }
}
