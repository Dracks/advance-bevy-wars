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

    /*pub fn offset(&self) -> IVec2 {
        match self {
            Direction::North => ivec2(0, -1),
            Direction::NorthEast => ivec2(1, -1),
            Direction::East => ivec2(1, 0),
            Direction::SouthEast => ivec2(1, 1),
            Direction::South => ivec2(0, 1),
            Direction::SouthWest => ivec2(-1, 1),
            Direction::West => ivec2(-1, 0),
            Direction::NorthWest => ivec2(-1, -1),
        }
    }*/

    const fn as_u32(self) -> u32 {
        self as u32
    }

    fn combine(list: &[Self]) -> u32 {
        list.iter().fold(0, |acc, layer| acc | layer.as_u32())
    }
}

pub struct Requirement<T> {
    terrain: T,
    mask: u32,
    not_mask: Option<u32>,
}

impl<T: Eq + Clone> Requirement<T> {
    pub fn new(terrain: T, directions: &Vec<Direction>) -> Self {
        println!("Requirement {:?} = {}", directions, Direction::combine(directions));
        Self {
            terrain,
            mask: Direction::combine(directions),
            not_mask: None,
        }
    }

    pub fn not_wanted(mut self, directions: &Vec<Direction>) -> Self {
        self.not_mask = Some(Direction::combine(directions));
        self
    }

    pub fn not_wanted_adj(mut self) -> Self {
        let adj = Direction::combine(&Direction::ADJACENT);
        self.not_mask = Some(adj & !self.mask);
        self
    }

    pub fn matches(&self, neighbors: &Vec<Neighbor<T>>) -> bool {
        let directions: Vec<_> = neighbors
            .iter()
            .filter(|neighbor| neighbor.terrain == self.terrain)
            .map(|neighbor| neighbor.direction)
            .collect();
        let combination = Direction::combine(&directions);
        let result = (combination & self.mask) == self.mask;
        let not_wanted = match self.not_mask {
            None => false,
            Some(not_mask) => {
                not_mask & combination > 0
            }
        };
        // println!("R matches: {combination} & {} ({}) = {} => {result} & {:?} => {not_wanted} ", self.mask, (combination&self.mask), self.mask, self.not_mask);
        result && ! not_wanted
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matches_work() {
        let subject = Requirement::new(1, &vec![Direction::North, Direction::East]);
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
        let subject = Requirement::new(1, &vec![]);
        let north_one = Neighbor::new(1, Direction::North);
        let south_one = Neighbor::new(1, Direction::South);
        let north_two = Neighbor::new(2, Direction::North);

        assert_eq!(subject.matches(&vec![north_one]), true);
        assert_eq!(subject.matches(&vec![north_two]), true);
        assert_eq!(subject.matches(&vec![south_one]), true);
    }

    #[test]
    fn test_not_directions() {
        let subject = Requirement::new(1, &vec![]).not_wanted(&vec![Direction::North, Direction::South]);
        let north_one = Neighbor::new(1, Direction::North);
        let south_one = Neighbor::new(1, Direction::South);
        let north_two = Neighbor::new(2, Direction::North);

        assert_eq!(subject.matches(&vec![north_one]), false);
        assert_eq!(subject.matches(&vec![north_two]), true);
        assert_eq!(subject.matches(&vec![south_one]), false);
    }

    #[test]
    fn test_not_adj_computed() {
        let subject = Requirement::new(1, &vec![Direction::South]).not_wanted_adj();
        let north_one = Neighbor::new(1, Direction::North);
        let south_one = Neighbor::new(1, Direction::South);
        let north_two = Neighbor::new(2, Direction::North);

        assert_eq!(subject.matches(&vec![north_one, south_one.clone()]), false);
        assert_eq!(subject.matches(&vec![north_two, south_one.clone()]), true);
        assert_eq!(subject.matches(&vec![south_one]), true);
    }
}
