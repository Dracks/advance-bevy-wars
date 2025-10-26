use std::collections::HashSet;

use auto_tiler::{AsMask, AutoTiler, Requirement, TileDefinition};
use bevy::{math::{uvec2, UVec2}};

use crate::board::board::Direction;

#[derive(Debug, Eq, Clone, Copy, PartialEq, Default, PartialOrd, Hash)]
pub enum Terrain {
    #[default]
    Plain,
    Sea,
    Mountain,
    Road,
    Beach,
    Bridge,
    Forest,
    Wall,
    BreakableWall,
}

impl From<&str> for Terrain {
    fn from(value: &str) -> Self {
        match value {
            "p" => Terrain::Plain,
            "s" => Terrain::Sea,
            "r" => Terrain::Road,
            "m" => Terrain::Mountain,
            "b" => Terrain::Bridge,
            "B" => Terrain::Beach,
            "w" => Terrain::Wall,
            "f" => Terrain::Forest,
            value => panic!("Value {value} unsupported"),
        }
    }
}

fn calculate(
    auto_tiler: &mut AutoTiler<Terrain, UVec2>,
    terrain: Terrain,
    neighbors: &HashSet<Terrain>,
    offset: &UVec2,
    tiles: &[UVec2],
    directions: &[Direction],
    not_wanted_reference: &[Direction],
) {
    for (quarter, tile) in tiles.iter().map(|x| x + offset).enumerate() {
        let rotation = (quarter * 2) as u8;
        let dirs: Vec<_> = directions.iter().map(|d| d.rotate_45(rotation)).collect();
        let not_wanted_rotate: Vec<_> = not_wanted_reference.iter().map(|d| d.rotate_45(rotation)).collect();
        auto_tiler.add_tile(
            TileDefinition::new(tile, terrain).add_possible_requirements(vec![
                Requirement::new(neighbors.clone(), &dirs).not_wanted_comp(&not_wanted_rotate),
            ]),
        );
    }
}

fn add_std_tiles(
    auto_tiler: &mut AutoTiler<Terrain, UVec2>,
    terrain: Terrain,
    offset: UVec2,
    neighbors: Option<Vec<Terrain>>,
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
        &Direction::ADJACENT,
    );

    // corner of lake
    calculate(
        auto_tiler,
        terrain,
        &neighbors,
        &offset,
        &[uvec2(0, 0), uvec2(2, 0), uvec2(2, 2), uvec2(0, 2)],
        &[Direction::South, Direction::SouthEast, Direction::East],
        &Direction::ADJACENT,
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
        &Direction::ADJACENT,
    );

    // Simple Corner
    calculate(
        auto_tiler,
        terrain,
        &neighbors,
        &offset,
        &[uvec2(4, 2), uvec2(5, 2), uvec2(5, 3), uvec2(4, 3)],
        &[Direction::South, Direction::East],
        &[Direction::North, Direction::West, Direction::SouthEast],
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
        &Direction::ALL,
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
        &Direction::ALL,
    )
}

fn add_mountain(auto_tiler: &mut AutoTiler<Terrain, UVec2>){
    let terrain = Terrain::Mountain;
    let neighbors = HashSet::from([terrain]);
    auto_tiler.add_tile(
        TileDefinition::new(uvec2(2, 25), terrain).add_possible_requirements(vec![
            Requirement::new_single::<Direction>(terrain, &[]).not_wanted(&Direction::ADJACENT),
        ]),
    );

    auto_tiler.add_tile(
        TileDefinition::new(uvec2(12, 22) , terrain)
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
        &Direction::ADJACENT,
    );

    // Sides of the mountain
    calculate(
        auto_tiler,
        terrain,
        &neighbors,
        &UVec2::ZERO,
        &[uvec2(12, 21), uvec2(13, 22), uvec2(12, 23), uvec2(11, 22)],
        &[Direction::South, Direction::West, Direction::East],
        &Direction::ADJACENT,
    );

    // Only one side
    calculate(
        auto_tiler,
        terrain,
        &neighbors,
        &UVec2::ZERO,
        &[uvec2(0, 23), uvec2(3, 21), uvec2(0, 24), uvec2(0, 21)],
        &[Direction::South],
        &Direction::ADJACENT,
    );

    // Two straight forward
    calculate(
        auto_tiler,
        terrain,
        &neighbors,
        &UVec2::ZERO,
        &[uvec2(1, 21), uvec2(0, 24)],
        &[Direction::East, Direction::West],
        &Direction::ADJACENT
    );
}

pub fn build_auto_tiler() -> AutoTiler<Terrain, UVec2> {
    let mut auto_tiler = AutoTiler::default();
    auto_tiler.add_tile(
        TileDefinition::new(uvec2(0, 15), Terrain::Plain)
            .add_possible_requirements(vec![Requirement::new::<Direction>(HashSet::new(), &[])]),
    );
    add_std_tiles(&mut auto_tiler, Terrain::Road, UVec2::ZERO, None);
    add_std_tiles(
        &mut auto_tiler,
        Terrain::Sea,
        uvec2(0, 5),
        Some(vec![Terrain::Sea, Terrain::Beach]),
    );
    add_std_tiles(
        &mut auto_tiler,
        Terrain::Beach,
        uvec2(0, 10),
        Some(vec![Terrain::Sea, Terrain::Beach]),
    );

    add_mountain(&mut auto_tiler);

    auto_tiler.add_tile(TileDefinition::new(uvec2(1,40), Terrain::Forest).add_possible_requirements(vec![Requirement::new::<Direction>(HashSet::new(), &[])]));

    auto_tiler
}
