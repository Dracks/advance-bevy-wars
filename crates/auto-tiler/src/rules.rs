use std::{collections::HashSet, hash::Hash};

use crate::board::Neighbor;

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
    pub const ALL: [Direction; 8] = [
        Direction::North,
        Direction::NorthEast,
        Direction::East,
        Direction::SouthEast,
        Direction::South,
        Direction::SouthWest,
        Direction::West,
        Direction::NorthWest,
    ];

    pub const ADJACENT: [Direction; 4] = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    const fn as_u32(self) -> u32 {
        self as u32
    }

    fn combine(list: &[Self]) -> u32 {
        list.iter().fold(0, |acc, layer| acc | layer.as_u32())
    }

    pub fn rotate_45(self, times: u8) -> Self {
        let bits = self.as_u32();
        let shift = (times % 8) as u32;

        // Fem rotate left: els bits que surten per l'esquerra entren per la dreta
        let new_bits = (bits << shift) | (bits >> (8 - shift));
        let moded = new_bits % (0b100000000);

        match moded {
            x if x == Direction::North.as_u32() => Direction::North,
            x if x == Direction::NorthEast.as_u32() => Direction::NorthEast,
            x if x == Direction::East.as_u32() => Direction::East,
            x if x == Direction::SouthEast.as_u32() => Direction::SouthEast,
            x if x == Direction::South.as_u32() => Direction::South,
            x if x == Direction::SouthWest.as_u32() => Direction::SouthWest,
            x if x == Direction::West.as_u32() => Direction::West,
            x if x == Direction::NorthWest.as_u32() => Direction::NorthWest,
            unknown => panic!("Something problematic happened processing rotate, we got {unknown}"),
        }
    }
}

pub struct Requirement<T> {
    terrains: HashSet<T>,
    mask: u32,
    not_mask: Option<u32>,
}

impl<T: Eq + Clone + Hash> Requirement<T> {
    pub fn new(terrains: HashSet<T>, directions: &[Direction]) -> Self {
        Self {
            terrains,
            mask: Direction::combine(directions),
            not_mask: None,
        }
    }

    pub fn not_wanted(mut self, directions: &[Direction]) -> Self {
        self.not_mask = Some(Direction::combine(directions));
        self
    }

    pub fn not_wanted_adj(self) -> Self {
        self.not_wanted_comp(&Direction::ADJACENT)
    }

    pub fn not_wanted_comp(mut self, directions: &[Direction]) -> Self {
        let adj = Direction::combine(directions);
        self.not_mask = Some(adj & !self.mask);
        self
    }

    pub fn matches(&self, neighbors: &Vec<Neighbor<T>>) -> bool {
        let directions: Vec<_> = neighbors
            .iter()
            .filter(|neighbor| self.terrains.contains(&neighbor.terrain))
            .map(|neighbor| neighbor.direction)
            .collect();
        let combination = Direction::combine(&directions);
        let result = (combination & self.mask) == self.mask;
        let not_wanted = match self.not_mask {
            None => false,
            Some(not_mask) => not_mask & combination > 0,
        };
        // println!("R matches: {combination} & {} ({}) = {} => {result} & {:?} => {not_wanted} ", self.mask, (combination&self.mask), self.mask, self.not_mask);
        result && !not_wanted
    }
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

    #[test]
    fn test_matches_work() {
        let subject =
            Requirement::new(HashSet::from([1]), &vec![Direction::North, Direction::East]);
        let north_one = Neighbor::new(1, Direction::North);
        let south_one = Neighbor::new(1, Direction::South);
        let north_two = Neighbor::new(2, Direction::North);
        let east_one = Neighbor::new(1, Direction::East);

        assert_eq!(
            subject.matches(&vec![north_one.clone(), east_one.clone()]),
            true
        );
        assert_eq!(subject.matches(&vec![north_two, east_one.clone()]), false);
        assert_eq!(
            subject.matches(&vec![north_one.clone(), south_one.clone()]),
            false
        );
        assert_eq!(subject.matches(&vec![north_one, east_one, south_one]), true);
    }

    #[test]
    fn test_no_directions() {
        let subject = Requirement::new(HashSet::from([1]), &vec![]);
        let north_one = Neighbor::new(1, Direction::North);
        let south_one = Neighbor::new(1, Direction::South);
        let north_two = Neighbor::new(2, Direction::North);

        assert_eq!(subject.matches(&vec![north_one]), true);
        assert_eq!(subject.matches(&vec![north_two]), true);
        assert_eq!(subject.matches(&vec![south_one]), true);
    }

    #[test]
    fn test_not_directions() {
        let subject = Requirement::new(HashSet::from([1]), &vec![])
            .not_wanted(&vec![Direction::North, Direction::South]);
        let north_one = Neighbor::new(1, Direction::North);
        let south_one = Neighbor::new(1, Direction::South);
        let north_two = Neighbor::new(2, Direction::North);

        assert_eq!(subject.matches(&vec![north_one]), false);
        assert_eq!(subject.matches(&vec![north_two]), true);
        assert_eq!(subject.matches(&vec![south_one]), false);
    }

    #[test]
    fn test_not_adj_computed() {
        let subject =
            Requirement::new(HashSet::from([1]), &vec![Direction::South]).not_wanted_adj();
        let north_one = Neighbor::new(1, Direction::North);
        let south_one = Neighbor::new(1, Direction::South);
        let north_two = Neighbor::new(2, Direction::North);

        assert_eq!(subject.matches(&vec![north_one, south_one.clone()]), false);
        assert_eq!(subject.matches(&vec![north_two, south_one.clone()]), true);
        assert_eq!(subject.matches(&vec![south_one]), true);
    }
}
