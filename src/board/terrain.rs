use bevy::{math::{uvec2, UVec2}, render::render_resource::AsBindGroupShaderType};
use auto_tiler::{AutoTiler, Direction, Requirement, TileDefinition};

#[derive(Debug, Eq, Clone, Copy, PartialEq, Default)]
pub enum Terrain {
    #[default]
    Plain,
    Sea,
    Mountain,
    Road,
}

impl Terrain {

    pub fn get_pos(&self)->UVec2 {
        match self {
            Terrain::Plain => uvec2(0,15),
            Terrain::Sea => uvec2(1, 6),
            Terrain::Road => uvec2(3, 3),
            Terrain::Mountain => uvec2(1, 21),
        }
    }
}

impl From<&str> for Terrain {
    fn from(value: &str) -> Self {
        match value {
            "p" => Terrain::Plain,
            "s" => Terrain::Sea,
            "r" => Terrain::Road,
            "m" => Terrain::Mountain,
            value => panic!("Value {value} unsupported"),
        }
    }
}

fn add_std_tiles(auto_tiler:&mut  AutoTiler<Terrain, UVec2>, terrain: Terrain, offset: UVec2){
    auto_tiler.add_tile(
        TileDefinition::new(uvec2(0,0)+offset, terrain)
            .add_possible_requirements(vec![Requirement::new(terrain, &vec![Direction::East, Direction::South, Direction::SouthWest])])
    ).add_tile(
        TileDefinition::new(uvec2(4,2)+offset, terrain)
            .add_possible_requirements(vec![Requirement::new(terrain, &vec![Direction::East, Direction::South])])
    ).add_tile(
        TileDefinition::new(uvec2(5,2)+offset, terrain)
            .add_possible_requirements(vec![Requirement::new(terrain, &vec![Direction::West, Direction::South])])
    )
    .add_tile(
        TileDefinition::new(uvec2(4,3)+offset, terrain)
            .add_possible_requirements(vec![Requirement::new(terrain, &vec![Direction::East, Direction::North])])
    )
    .add_tile(
        TileDefinition::new(uvec2(5,3)+offset, terrain)
            .add_possible_requirements(vec![Requirement::new(terrain, &vec![Direction::West, Direction::North])])
    )
    .add_tile(TileDefinition::new(uvec2(3,3)+offset, terrain)
        .add_possible_requirements(vec![Requirement::new(terrain, &vec![])]));
}

pub fn build_auto_tiler() -> AutoTiler<Terrain, UVec2> {
    let mut auto_tiler = AutoTiler::default();
    auto_tiler.add_tile(TileDefinition::new(uvec2(0,15), Terrain::Plain).add_possible_requirements(vec![Requirement::new(Terrain::Plain, &vec![])]));
    add_std_tiles(&mut auto_tiler, Terrain::Road, UVec2::ZERO);
    add_std_tiles(&mut auto_tiler, Terrain::Sea, uvec2(0,5));

    auto_tiler
}
