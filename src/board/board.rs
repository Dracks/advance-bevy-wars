use auto_tiler::{AutoTiler, BoardTrait, Direction, Neighbor};
use bevy::prelude::*;
use rand::seq::IndexedRandom;

use crate::{assets::FileAssets, board::terrain::{build_auto_tiler, Terrain}};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Board::random(UVec2::splat(20)))
            .insert_resource(Tiler(build_auto_tiler()))
            .add_systems(Startup, spawn_terrain);
    }
}


#[derive(Resource)]
pub struct Tiler(AutoTiler<Terrain, UVec2>);

#[derive(Resource)]
pub struct Board {
    width: usize,
    height: usize,
    tiles: Vec<Vec<Terrain>>,
}

impl Board {
    pub fn get_size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn fill(width: usize, height:usize, terrain: Terrain) -> Self {
        Self {
            width,
            height,
            tiles: vec![vec![terrain; width]; height],
        }
    }

    pub fn get_all(&self) -> Vec<UVec2> {
        let mut all_linear = Vec::with_capacity(self.width*self.height);
        for x in 0..self.width {
            for y in 0..self.height {
                all_linear.push(uvec2(x as u32,y as u32))
            }
        }
        all_linear
    }

    pub fn random(size: UVec2) -> Self {
        let width= size.x as usize;
        let height= size.y as usize;
        let terrains = vec![Terrain::Mountain, Terrain::Plain, Terrain::Road, Terrain::Sea];
        let mut tiles = Vec::with_capacity(height);
        let mut rng = rand::rng();
        for _ in 0..size.y {
            let mut row = Vec::with_capacity(width);
            for _ in 0..size.x{
                match terrains.choose(&mut rng) {
                    Some(terrain) => row.push(terrain.clone()),
                    None => panic!("No terrain picked?")
                }
            }
            tiles.push(row);
        }
        Self {
            width,
            height,
            tiles
        }
    }
}

impl BoardTrait<Terrain, UVec2> for Board {
    fn get(&self, pos: &UVec2) -> Option<&Terrain> {
        let x = pos.x as usize;
        let y = pos.y as usize;
        if x>=self.width || y >= self.height {
            return None
        }
        Some(&self.tiles[pos.y as usize][pos.x as usize])
    }

    fn get_neighbors(&self, pos: &UVec2, directions: &[Direction]) -> Vec<Neighbor<Terrain>> {
        directions.iter()
            .filter_map(|dir| {
                let neighbor_pos = match dir {
                    Direction::North => uvec2(pos.x, pos.y.checked_sub(1)?),
                    Direction::South => uvec2(pos.x, pos.y + 1),
                    Direction::East => uvec2(pos.x + 1, pos.y),
                    Direction::West => uvec2(pos.x.checked_sub(1)?, pos.y),
                    _ => return None,
                };
                self.get(&neighbor_pos).map(|terrain| Neighbor::new(*terrain, *dir))
            })
            .collect()
    }
}

impl From<Vec<Vec<&str>>> for Board {
    fn from(value: Vec<Vec<&str>>) -> Self {
        let height = value.len();
        assert!(height>0, "The value must contain data (Height = 0)");
        let width = value[0].len();
        assert!(width>0, "The value must contain data (Width = 0)");
        let mut tiles = Vec::with_capacity(height);
        for y in 0..height {
            bevy::log::info!("Transforming y {y}");
            let row = value[y].clone();
            assert_eq!(row.len(), width, "Row {y} has an invalid width");
            tiles.push(row.iter().map(|text| Terrain::from(*text)).collect());
        }
        bevy::log::info!("Transforming {width} {height}");
        Self {
            width,
            height,
            tiles,
        }
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


fn spawn_terrain(mut commands: Commands, assets: Res<AssetServer>, auto_tiler: Res<Tiler>, mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>, board: Res<Board>){
    let helper = TileHelper::new(uvec2(68, 45));

    let texture_handle = FileAssets::ImagesGameTerrain.load(&assets);
    let texture_atlas = helper.atlas_layout(UVec2::splat(32));
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let auto_tiler : &AutoTiler<Terrain, UVec2> = &auto_tiler.0;

    for pos in board.get_all() {
        if let Some(tile_coords) = auto_tiler.get_tile::<UVec2>(&*board, pos){
            commands.spawn((
                Sprite::from_atlas_image(texture_handle.clone(), TextureAtlas {
                    layout: texture_atlas_handle.clone(),
                    index: helper.index(tile_coords),
                }),
                Transform::from_translation(vec3((pos.x*32) as f32, (pos.y*32) as f32, 0.))
            ));
        }
    }
}
