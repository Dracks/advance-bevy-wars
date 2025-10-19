use auto_tiler::{AutoTiler, Direction, Requirement, TileDefinition};
use bevy::math::{UVec2, uvec2};

#[derive(Debug, Eq, Clone, Copy, PartialEq, Default, PartialOrd)]
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
            value => panic!("Value {value} unsupported"),
        }
    }
}

fn add_std_tiles(auto_tiler: &mut AutoTiler<Terrain, UVec2>, terrain: Terrain, offset: UVec2) {
    auto_tiler
        .add_tile(
            TileDefinition::new(uvec2(0, 0) + offset, terrain).add_possible_requirements(vec![
                Requirement::new(
                    terrain,
                    &vec![Direction::East, Direction::South, Direction::SouthEast],
                ).not_wanted_adj(),
            ]),
        )
        .add_tile(
            TileDefinition::new(uvec2(0, 1) + offset, terrain).add_possible_requirements(vec![
                Requirement::new(
                    terrain,
                    &vec![Direction::North, Direction::South, Direction::East],
                ).not_wanted_adj(),
            ]),
        )
        .add_tile(
            TileDefinition::new(uvec2(0, 2) + offset, terrain).add_possible_requirements(vec![
                Requirement::new(
                    terrain,
                    &vec![Direction::East, Direction::North, Direction::NorthEast],
                ).not_wanted_adj(),
            ]),
        )
        .add_tile(
            TileDefinition::new(uvec2(3, 1) + offset, terrain).add_possible_requirements(vec![
                Requirement::new(terrain, &vec![Direction::North, Direction::South]).not_wanted_adj(),
            ]),
        )
        .add_tile(
            TileDefinition::new(uvec2(1, 3) + offset, terrain).add_possible_requirements(vec![
                Requirement::new(terrain, &vec![Direction::East, Direction::West]).not_wanted_adj(),
            ]),
        )
        .add_tile(
            TileDefinition::new(uvec2(3, 0) + offset, terrain).add_possible_requirements(vec![
                Requirement::new(terrain, &vec![Direction::South]).not_wanted_adj(),
            ]),
        )
        .add_tile(
            TileDefinition::new(uvec2(3, 2) + offset, terrain).add_possible_requirements(vec![
                Requirement::new(terrain, &vec![Direction::North]).not_wanted_adj(),
            ]),
        )
        .add_tile(
            TileDefinition::new(uvec2(0, 3) + offset, terrain)
                .add_possible_requirements(vec![Requirement::new(terrain, &vec![Direction::East]).not_wanted_adj()]),
        )
        .add_tile(
            TileDefinition::new(uvec2(2, 4) + offset, terrain)
                .add_possible_requirements(vec![Requirement::new(terrain, &vec![Direction::West]).not_wanted_adj()]),
        )
        .add_tile(
            TileDefinition::new(uvec2(4, 2) + offset, terrain).add_possible_requirements(vec![
                Requirement::new(terrain, &vec![Direction::East, Direction::South]).not_wanted_adj(),
            ]),
        )
        .add_tile(
            TileDefinition::new(uvec2(5, 2) + offset, terrain).add_possible_requirements(vec![
                Requirement::new(terrain, &vec![Direction::West, Direction::South]).not_wanted_adj(),
            ]),
        )
        .add_tile(
            TileDefinition::new(uvec2(4, 3) + offset, terrain).add_possible_requirements(vec![
                Requirement::new(terrain, &vec![Direction::East, Direction::North]).not_wanted_adj(),
            ]),
        )
        .add_tile(
            TileDefinition::new(uvec2(5, 3) + offset, terrain).add_possible_requirements(vec![
                Requirement::new(terrain, &vec![Direction::West, Direction::North]).not_wanted_adj(),
            ]),
        )
        .add_tile(
            TileDefinition::new(uvec2(3, 3) + offset, terrain)
                .add_possible_requirements(vec![Requirement::new(terrain, &vec![]).not_wanted_adj()]),
        );
}

fn permutate_terrains(definitions: Vec<&mut TileDefinition>, terrains: Terrain, directions: &Vec<Direction>){
    for definition in definitions.iter_mut() {

    }
}

pub fn build_auto_tiler() -> AutoTiler<Terrain, UVec2> {
    let mut auto_tiler = AutoTiler::default();
    auto_tiler.add_tile(
        TileDefinition::new(uvec2(0, 15), Terrain::Plain)
            .add_possible_requirements(vec![Requirement::new(Terrain::Plain, &vec![])]),
    );
    add_std_tiles(&mut auto_tiler, Terrain::Road, UVec2::ZERO);
    add_std_tiles(&mut auto_tiler, Terrain::Sea, uvec2(0, 5));

    auto_tiler
}
