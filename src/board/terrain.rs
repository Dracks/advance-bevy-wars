use bevy::math::{uvec2, UVec2};


#[derive(Debug, Clone, Copy, PartialEq, Default)]
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
