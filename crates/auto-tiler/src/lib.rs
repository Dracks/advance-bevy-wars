pub mod auto_tiler;
pub mod board;
pub mod rules;

pub use crate::auto_tiler::*;
pub use crate::board::*;
pub use crate::rules::*;

pub mod direction {
    use crate::AsMask;


    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum AdjacentDirection {
        North = 0b00000001,
        East = 0b00000010,
        South = 0b00000100,
        West = 0b00001000,
    }


    impl AsMask for AdjacentDirection {
        fn as_mask(self) -> u32 {
            self as u32
        }

        const ALL: &'static [Self] = &[
            AdjacentDirection::North,
            AdjacentDirection::East,
            AdjacentDirection::South,
            AdjacentDirection::West,
        ];
    }
}
