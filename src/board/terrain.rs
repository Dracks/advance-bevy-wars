use std::collections::HashSet;

use auto_tiler::{AsMask, AutoTiler, Requirement, TileDefinition};
use bevy::math::{UVec2, uvec2};

use crate::board::{direction::Direction, map::Terrain, map::Terrain::*};

#[derive(Debug, Eq, Clone, Copy, PartialEq, Default, PartialOrd, Hash)]
pub enum TileTerrain {
    #[default]
    Plain,
    Sea,
    Mountain,
    Road,
    Beach,
    Bridge,
    Forest,
    Wall,
    // BreakableWall,
}

impl From<&Terrain> for TileTerrain {
    fn from(value: &Terrain) -> Self {
        match value {
            Plane => Self::Plain,
            Sea => Self::Sea,
            Road => Self::Road,
            Mountain => Self::Mountain,
            //"b" => Self::Bridge,
            Beach => Self::Beach,
            //"w" => Self::Wall,
            Forest => Self::Forest,
        }
    }
}

enum NotWanted<'a> {
    Computed(&'a [Direction]),
    Rotated(&'a [Direction]),
}

fn calculate(
    auto_tiler: &mut AutoTiler<TileTerrain, UVec2>,
    terrain: TileTerrain,
    neighbors: &HashSet<TileTerrain>,
    offset: &UVec2,
    tiles: &[UVec2],
    directions: &[Direction],
    not_wanted_reference: &NotWanted,
) {
    for (quarter, tile) in tiles.iter().map(|x| x + offset).enumerate() {
        let rotation = (quarter * 2) as u8;
        let dirs: Vec<_> = directions.iter().map(|d| d.rotate_45(rotation)).collect();
        let mut requirement = Requirement::new(neighbors.clone(), &dirs);
        match not_wanted_reference {
            NotWanted::Computed(directions) => {
                requirement = requirement.not_wanted_comp(directions)
            }
            NotWanted::Rotated(directions) => {
                let rotate_directions: Vec<_> =
                    directions.iter().map(|d| d.rotate_45(rotation)).collect();
                requirement = requirement.not_wanted(&rotate_directions)
            }
        };
        auto_tiler.add_tile(
            TileDefinition::new(tile, terrain).add_possible_requirements(vec![requirement]),
        );
    }
}

fn add_std_tiles(
    auto_tiler: &mut AutoTiler<TileTerrain, UVec2>,
    terrain: TileTerrain,
    offset: UVec2,
    neighbors: Option<Vec<TileTerrain>>,
) {
    let neighbors = match neighbors {
        None => HashSet::from([terrain]),
        Some(opts) => HashSet::from_iter(opts),
    };
    auto_tiler.add_tile(
        TileDefinition::new(uvec2(3, 3) + offset, terrain).add_possible_requirements(vec![
            Requirement::new::<Direction>(neighbors.clone(), &[]).not_wanted(&Direction::ADJACENT),
        ]),
    );

    auto_tiler.add_tile(
        TileDefinition::new(uvec2(1, 1) + offset, terrain)
            .add_possible_requirements(vec![Requirement::new(neighbors.clone(), &Direction::ALL)]),
    );

    auto_tiler.add_tile(
        TileDefinition::new(uvec2(10, 3) + offset, terrain).add_possible_requirements(vec![
            Requirement::new(neighbors.clone(), &Direction::ADJACENT)
                .not_wanted_comp(Direction::ALL),
        ]),
    );

    // Start end
    calculate(
        auto_tiler,
        terrain,
        &neighbors,
        &offset,
        &[uvec2(3, 2), uvec2(0, 3), uvec2(3, 0), uvec2(2, 3)],
        &[Direction::North],
        &NotWanted::Computed(&Direction::ADJACENT),
    );

    // corner of lake
    calculate(
        auto_tiler,
        terrain,
        &neighbors,
        &offset,
        &[uvec2(0, 0), uvec2(2, 0), uvec2(2, 2), uvec2(0, 2)],
        &[Direction::South, Direction::SouthEast, Direction::East],
        &NotWanted::Rotated(&[Direction::North, Direction::West]),
    );

    // border of lake
    calculate(
        auto_tiler,
        terrain,
        &neighbors,
        &offset,
        &[uvec2(1, 0), uvec2(2, 1), uvec2(1, 2), uvec2(0, 1)],
        &[
            Direction::South,
            Direction::East,
            Direction::West,
            Direction::SouthEast,
            Direction::SouthWest,
        ],
        &NotWanted::Computed(&Direction::ADJACENT),
    );

    // Simple Corner
    calculate(
        auto_tiler,
        terrain,
        &neighbors,
        &offset,
        &[uvec2(4, 2), uvec2(5, 2), uvec2(5, 3), uvec2(4, 3)],
        &[Direction::South, Direction::East],
        &NotWanted::Rotated(&[Direction::North, Direction::West, Direction::SouthEast]),
    );

    // simple straight forward
    for (half, tile) in [uvec2(3, 1), uvec2(1, 3)]
        .iter()
        .map(|x| x + offset)
        .enumerate()
    {
        let rotation = (half * 2) as u8;
        let directions: Vec<_> = [Direction::North, Direction::South]
            .iter()
            .map(|x| x.rotate_45(rotation))
            .collect();
        auto_tiler.add_tile(
            TileDefinition::new(tile, terrain).add_possible_requirements(vec![
                Requirement::new(neighbors.clone(), &directions)
                    .not_wanted_comp(&Direction::ADJACENT),
            ]),
        );
    }

    // 3 ways
    calculate(
        auto_tiler,
        terrain,
        &neighbors,
        &offset,
        &[uvec2(6, 1), uvec2(9, 0), uvec2(7, 1), uvec2(9, 1)],
        &[Direction::South, Direction::East, Direction::North],
        &NotWanted::Computed(&Direction::ALL),
    );

    // All Except one corner
    calculate(
        auto_tiler,
        terrain,
        &neighbors,
        &offset,
        &[uvec2(5, 0), uvec2(5, 1), uvec2(4, 1), uvec2(4, 0)],
        &[
            Direction::North,
            Direction::NorthEast,
            Direction::East,
            Direction::SouthEast,
            Direction::South,
            Direction::West,
            Direction::NorthWest,
        ],
        &NotWanted::Computed(Direction::ALL),
    )
}

fn add_mountain(auto_tiler: &mut AutoTiler<TileTerrain, UVec2>) {
    let terrain = TileTerrain::Mountain;
    let neighbors = HashSet::from([terrain]);
    auto_tiler.add_tile(
        TileDefinition::new(uvec2(2, 25), terrain).add_possible_requirements(vec![
            Requirement::new_single::<Direction>(terrain, &[]).not_wanted(&Direction::ADJACENT),
        ]),
    );

    auto_tiler.add_tile(
        TileDefinition::new(uvec2(12, 22), terrain)
            .add_possible_requirements(vec![Requirement::new_single(terrain, &Direction::ALL)]),
    );

    // corner of mountain
    calculate(
        auto_tiler,
        terrain,
        &neighbors,
        &UVec2::ZERO,
        &[uvec2(11, 21), uvec2(13, 21), uvec2(13, 23), uvec2(11, 23)],
        &[Direction::South, Direction::SouthEast, Direction::East],
        &NotWanted::Rotated(&[Direction::North, Direction::West]),
    );

    // Sides of the mountain
    calculate(
        auto_tiler,
        terrain,
        &neighbors,
        &UVec2::ZERO,
        &[uvec2(12, 21), uvec2(13, 22), uvec2(12, 23), uvec2(11, 22)],
        &[Direction::South, Direction::West, Direction::East],
        &NotWanted::Rotated(&[Direction::North]),
    );

    // Only one side
    calculate(
        auto_tiler,
        terrain,
        &neighbors,
        &UVec2::ZERO,
        &[uvec2(0, 23), uvec2(3, 21), uvec2(0, 24), uvec2(0, 21)],
        &[Direction::South],
        &NotWanted::Computed(&Direction::ADJACENT),
    );

    // Two straight forward
    calculate(
        auto_tiler,
        terrain,
        &neighbors,
        &UVec2::ZERO,
        &[uvec2(1, 21), uvec2(0, 24)],
        &[Direction::East, Direction::West],
        &NotWanted::Computed(&Direction::ADJACENT),
    );
}

pub fn build_auto_tiler() -> AutoTiler<TileTerrain, UVec2> {
    let mut auto_tiler = AutoTiler::default();
    auto_tiler.add_tile(
        TileDefinition::new(uvec2(0, 15), TileTerrain::Plain)
            .add_possible_requirements(vec![Requirement::new::<Direction>(HashSet::new(), &[])]),
    );
    add_std_tiles(&mut auto_tiler, TileTerrain::Road, UVec2::ZERO, None);
    add_std_tiles(
        &mut auto_tiler,
        TileTerrain::Sea,
        uvec2(0, 5),
        Some(vec![TileTerrain::Sea, TileTerrain::Beach]),
    );
    add_std_tiles(
        &mut auto_tiler,
        TileTerrain::Beach,
        uvec2(0, 10),
        Some(vec![TileTerrain::Sea, TileTerrain::Beach]),
    );

    add_mountain(&mut auto_tiler);

    auto_tiler.add_tile(
        TileDefinition::new(uvec2(1, 40), TileTerrain::Forest)
            .add_possible_requirements(vec![Requirement::new::<Direction>(HashSet::new(), &[])]),
    );

    auto_tiler
}
