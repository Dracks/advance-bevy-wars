use bevy::prelude::*;

use crate::assets::FileAssets;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Board::default())
            .add_systems(Startup, spawn_terrain);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Terrain {
    #[default]
    Plain,
    Sea,
    Mountain,
    Road
}

impl Terrain {
    fn get_pos(&self)->UVec2 {
        match self {
            Terrain::Plain => uvec2(0,15),
            _ => uvec2(0,0),
        }
    }
}


#[derive(Resource)]
pub struct Board {
    width: usize,
    height: usize,
    tiles: Vec<Vec<Terrain>>,
}

impl Board {
    pub fn fill(width: usize, height:usize, terrain: Terrain) -> Self {
        Self {
            width,
            height,
            tiles: vec![vec![terrain; width]; height],
        }
    }

    pub fn get_all(&self) -> Vec<(UVec2, Terrain)> {
        let mut all_linear = Vec::with_capacity(self.width*self.height);
        for x in 0..self.width {
            for y in 0..self.height {
                all_linear.push((uvec2(x as u32,y as u32), self.tiles[x][y]))
            }
        }
        all_linear
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::fill(20, 20, Terrain::Plain)
    }
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


fn spawn_terrain(mut commands: Commands, assets: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>, board: Res<Board>){
    let helper = TileHelper::new(uvec2(68, 45));

    let texture_handle = FileAssets::ImagesGameTerrain.load(&assets);
    let texture_atlas = helper.atlas_layout(UVec2::splat(32));
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    for (pos, terrain) in board.get_all() {
        commands.spawn((
            Sprite::from_atlas_image(texture_handle.clone(), TextureAtlas {
                layout: texture_atlas_handle.clone(),
                index: helper.index(terrain.get_pos()),
            }),
            Transform::from_translation(vec3((pos.x*32) as f32, (pos.y*32) as f32, 0.))
        ));
    }
}
