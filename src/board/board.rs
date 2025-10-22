use auto_tiler::{AsMask, AutoTiler, BoardTrait, Neighbor};
use bevy::prelude::*;
use rand::seq::IndexedRandom;

use crate::{
    assets::FileAssets,
    board::{
        samples::base_board,
        terrain::{Terrain, build_auto_tiler},
    },
};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North = 0b00000001,
    NorthEast = 0b00000010,
    East = 0b00000100,
    SouthEast = 0b00001000,
    South = 0b00010000,
    SouthWest = 0b00100000,
    West = 0b01000000,
    NorthWest = 0b10000000,
}

impl Direction {

    pub const ADJACENT: [Direction; 4] = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    pub fn rotate_45(self, times: u8) -> Self {
        let bits = self.as_mask();
        let shift = (times % 8) as u32;

        // Fem rotate left: els bits que surten per l'esquerra entren per la dreta
        let new_bits = (bits << shift) | (bits >> (8 - shift));
        let moded = new_bits % (0b100000000);

        match moded {
            x if x == Direction::North.as_mask() => Direction::North,
            x if x == Direction::NorthEast.as_mask() => Direction::NorthEast,
            x if x == Direction::East.as_mask() => Direction::East,
            x if x == Direction::SouthEast.as_mask() => Direction::SouthEast,
            x if x == Direction::South.as_mask() => Direction::South,
            x if x == Direction::SouthWest.as_mask() => Direction::SouthWest,
            x if x == Direction::West.as_mask() => Direction::West,
            x if x == Direction::NorthWest.as_mask() => Direction::NorthWest,
            unknown => panic!("Something problematic happened processing rotate, we got {unknown}"),
        }
    }
}

impl AsMask for Direction {
    fn as_mask(self) -> u32 {
        self as u32
    }

    const ALL: &'static [Self] = &[
        Direction::North,
        Direction::NorthEast,
        Direction::East,
        Direction::SouthEast,
        Direction::South,
        Direction::SouthWest,
        Direction::West,
        Direction::NorthWest,
    ];
}

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Board::random(uvec2(50, 30)))
            .insert_resource(Tiler(build_auto_tiler()))
            .add_systems(Startup, spawn_terrain);
        app.insert_resource(base_board());
    }
}

#[derive(Component)]
pub struct MainBoard;

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

    pub fn fill(width: usize, height: usize, terrain: Terrain) -> Self {
        Self {
            width,
            height,
            tiles: vec![vec![terrain; width]; height],
        }
    }

    pub fn get_all(&self) -> Vec<UVec2> {
        let mut all_linear = Vec::with_capacity(self.width * self.height);
        for x in 0..self.width {
            for y in 0..self.height {
                all_linear.push(uvec2(x as u32, y as u32))
            }
        }
        all_linear
    }

    pub fn random(size: UVec2) -> Self {
        let width = size.x as usize;
        let height = size.y as usize;
        let terrains = vec![
            Terrain::Mountain,
            Terrain::Plain,
            Terrain::Road,
            Terrain::Sea,
            Terrain::Beach,
            Terrain::Forest,
        ];
        let mut tiles = Vec::with_capacity(height);
        let mut rng = rand::rng();
        for _ in 0..size.y {
            let mut row = Vec::with_capacity(width);
            for _ in 0..size.x {
                match terrains.choose(&mut rng) {
                    Some(terrain) => row.push(terrain.clone()),
                    None => panic!("No terrain picked?"),
                }
            }
            tiles.push(row);
        }
        Self {
            width,
            height,
            tiles,
        }
    }
}

impl BoardTrait<Terrain, UVec2, Direction> for Board {
    fn get(&self, pos: &UVec2) -> Option<&Terrain> {
        let x = pos.x as usize;
        let y = pos.y as usize;
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(&self.tiles[pos.y as usize][pos.x as usize])
    }

    fn get_neighbors(&self, pos: &UVec2, directions: &[Direction]) -> Vec<Neighbor<Terrain, Direction>> {
        directions
            .iter()
            .filter_map(|dir| {
                let neighbor_pos = match dir {
                    Direction::North => uvec2(pos.x, pos.y + 1),
                    Direction::South => uvec2(pos.x, pos.y.checked_sub(1)?),
                    Direction::East => uvec2(pos.x + 1, pos.y),
                    Direction::West => uvec2(pos.x.checked_sub(1)?, pos.y),
                    Direction::NorthEast => uvec2(pos.x + 1, pos.y + 1),
                    Direction::NorthWest => uvec2(pos.x.checked_sub(1)?, pos.y + 1),
                    Direction::SouthEast => uvec2(pos.x + 1, pos.y.checked_sub(1)?),
                    Direction::SouthWest => uvec2(pos.x.checked_sub(1)?, pos.y.checked_sub(1)?),
                };
                self.get(&neighbor_pos)
                    .map(|terrain| Neighbor::new(*terrain, *dir))
            })
            .collect()
    }
}

impl From<Vec<Vec<&str>>> for Board {
    fn from(value: Vec<Vec<&str>>) -> Self {
        let height = value.len();
        assert!(height > 0, "The value must contain data (Height = 0)");
        let width = value[0].len();
        assert!(width > 0, "The value must contain data (Width = 0)");
        let mut tiles = Vec::with_capacity(height);
        for y in 0..height {
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
        Self { size }
    }

    pub fn index(&self, pos: UVec2) -> usize {
        assert!(self.size.x >= pos.x);
        assert!(self.size.y >= pos.y);
        return (pos.y * self.size.x + pos.x) as usize;
    }

    pub fn atlas_layout(&self, tile_size: UVec2) -> TextureAtlasLayout {
        TextureAtlasLayout::from_grid(tile_size, self.size.x, self.size.y, None, None)
    }
}

fn spawn_terrain(
    mut commands: Commands,
    assets: Res<AssetServer>,
    auto_tiler: Res<Tiler>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    board: Res<Board>,
) {
    let helper = TileHelper::new(uvec2(68, 45));

    let texture_handle = FileAssets::ImagesGameTerrain.load(&assets);
    let texture_atlas = helper.atlas_layout(UVec2::splat(32));
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let auto_tiler: &AutoTiler<Terrain, UVec2> = &auto_tiler.0;
    commands
        .spawn((Transform::IDENTITY, Visibility::Inherited, MainBoard))
        .with_children(|parent| {
            for pos in board.get_all() {
                if let Some(tile_coords) = auto_tiler.get_tile::<UVec2, Direction>(&*board, pos) {
                    parent.spawn((
                        Sprite::from_atlas_image(
                            texture_handle.clone(),
                            TextureAtlas {
                                layout: texture_atlas_handle.clone(),
                                index: helper.index(tile_coords),
                            },
                        ),
                        Transform::from_translation(vec3(
                            (pos.x * 32) as f32,
                            (pos.y * 32) as f32,
                            0.,
                        )),
                    ));
                }
            }
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        assert_eq!(Direction::North.rotate_45(1), Direction::NorthEast);
        assert_eq!(Direction::West.rotate_45(4), Direction::East);
        assert_eq!(Direction::West.rotate_45(8), Direction::West);
        assert_eq!(Direction::West.rotate_45(12), Direction::East);
        assert_eq!(Direction::West.rotate_45(1), Direction::NorthWest);
    }
}
