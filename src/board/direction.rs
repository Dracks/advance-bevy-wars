use auto_tiler::AsMask;
use bevy::math::{UVec2, Vec2, uvec2, vec2};

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

    pub fn move_point(self, pos: &UVec2) -> Option<UVec2> {
        let data = match self {
            Direction::North => uvec2(pos.x, pos.y + 1),
            Direction::South => uvec2(pos.x, pos.y.checked_sub(1)?),
            Direction::East => uvec2(pos.x + 1, pos.y),
            Direction::West => uvec2(pos.x.checked_sub(1)?, pos.y),
            Direction::NorthEast => uvec2(pos.x + 1, pos.y + 1),
            Direction::NorthWest => uvec2(pos.x.checked_sub(1)?, pos.y + 1),
            Direction::SouthEast => uvec2(pos.x + 1, pos.y.checked_sub(1)?),
            Direction::SouthWest => uvec2(pos.x.checked_sub(1)?, pos.y.checked_sub(1)?),
        };
        Some(data)
    }

    #[allow(dead_code)]
    pub fn as_vec2(self) -> Vec2 {
        match self {
            Direction::North => vec2(0., 1.),
            Direction::South => vec2(0., -1.),
            Direction::East => vec2(1., 0.),
            Direction::West => vec2(-1., 0.),
            Direction::NorthEast => vec2(1., 1.),
            Direction::NorthWest => vec2(-1., 1.),
            Direction::SouthEast => vec2(1., -1.),
            Direction::SouthWest => vec2(-1., -1.),
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
