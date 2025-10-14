use bevy::prelude::*;
use bevy_flair::FlairPlugin;

mod assets;
use assets::FileAssets;

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
    .add_systems(Startup, setup);

    app.run();
}

struct TileHelper {
    size: UVec2,
}

impl TileHelper {
    pub fn new(size: UVec2) -> Self {
        Self {
            size,
        }
    }

    pub fn index(&self, pos: UVec2) -> usize {
        assert!(self.size.x>=pos.x);
        assert!(self.size.y>=pos.y);
        return (pos.y*self.size.x+pos.x) as usize
    }

    pub fn atlas_layout(&self, tile_size: UVec2) -> TextureAtlasLayout {
        TextureAtlasLayout::from_grid(tile_size, self.size.x, self.size.y, None, None)
    }
}

fn setup(mut commands: Commands, assets: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,){
    commands.spawn((Camera2d, Transform::IDENTITY));

    let helper = TileHelper::new(uvec2(68, 45));

    let texture_handle = FileAssets::ImagesGameTerrain.load(&assets);
    let texture_atlas = helper.atlas_layout(UVec2::splat(32));
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn(
        Sprite::from_atlas_image(texture_handle, TextureAtlas {
            layout: texture_atlas_handle,
            index: helper.index(uvec2(0, 5)),
        }),
    );
}
