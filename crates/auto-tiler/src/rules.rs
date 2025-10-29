use std::{collections::HashSet, hash::Hash};

use crate::board::Neighbor;
pub trait AsMask: Copy
where
    Self: 'static,
{
    const ALL: &'static [Self];
    fn as_mask(self) -> u32;
    fn combine(list: &[impl AsMask]) -> u32 {
        list.iter().fold(0, |acc, layer| acc | layer.as_mask())
    }
}

pub struct Requirement<T> {
    terrains: HashSet<T>,
    mask: u32,
    not_mask: Option<u32>,
}

impl<T: Eq + Clone + Hash> Requirement<T> {
    pub fn new<D: AsMask>(terrains: HashSet<T>, directions: &[D]) -> Self {
        Self {
            terrains,
            mask: D::combine(directions),
            not_mask: None,
        }
    }

    pub fn new_single<D: AsMask>(terrain: T, directions: &[D]) -> Self {
        Self::new(HashSet::from([terrain]), directions)
    }

    pub fn not_wanted<D: AsMask>(mut self, directions: &[D]) -> Self {
        let not_mask = D::combine(directions);
        assert_eq!(
            not_mask & self.mask,
            0,
            "Not wanted directions cannot contain a required direction"
        );
        self.not_mask = Some(not_mask);
        self
    }

    pub fn not_wanted_comp<D: AsMask>(mut self, directions: &[D]) -> Self {
        let adj = D::combine(directions);
        self.not_mask = Some(adj & !self.mask);
        self
    }

    pub fn matches<D: AsMask>(&self, neighbors: &[Neighbor<T, D>]) -> bool {
        let directions: Vec<_> = neighbors
            .iter()
            .filter(|neighbor| self.terrains.contains(&neighbor.terrain))
            .map(|neighbor| neighbor.direction)
            .collect();
        let combination = D::combine(&directions);
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
    use crate::direction::AdjacentDirection;

    use super::*;

    #[test]
    fn test_matches_work() {
        let subject = Requirement::new(
            HashSet::from([1]),
            &vec![AdjacentDirection::North, AdjacentDirection::East],
        );
        let north_one = Neighbor::new(1, AdjacentDirection::North);
        let south_one = Neighbor::new(1, AdjacentDirection::South);
        let north_two = Neighbor::new(2, AdjacentDirection::North);
        let east_one = Neighbor::new(1, AdjacentDirection::East);

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
        let subject = Requirement::new::<AdjacentDirection>(HashSet::from([1]), &vec![]);
        let north_one = Neighbor::new(1, AdjacentDirection::North);
        let south_one = Neighbor::new(1, AdjacentDirection::South);
        let north_two = Neighbor::new(2, AdjacentDirection::North);

        assert_eq!(subject.matches(&vec![north_one]), true);
        assert_eq!(subject.matches(&vec![north_two]), true);
        assert_eq!(subject.matches(&vec![south_one]), true);
    }

    #[test]
    fn test_not_directions() {
        let subject = Requirement::new::<AdjacentDirection>(HashSet::from([1]), &vec![])
            .not_wanted(&vec![AdjacentDirection::North, AdjacentDirection::South]);
        let north_one = Neighbor::new(1, AdjacentDirection::North);
        let south_one = Neighbor::new(1, AdjacentDirection::South);
        let north_two = Neighbor::new(2, AdjacentDirection::North);

        assert_eq!(subject.matches(&vec![north_one]), false);
        assert_eq!(subject.matches(&vec![north_two]), true);
        assert_eq!(subject.matches(&vec![south_one]), false);
    }

    #[test]
    fn test_not_adj_computed() {
        let subject = Requirement::new(HashSet::from([1]), &[AdjacentDirection::South])
            .not_wanted_comp(AdjacentDirection::ALL);
        let north_one = Neighbor::new(1, AdjacentDirection::North);
        let south_one = Neighbor::new(1, AdjacentDirection::South);
        let north_two = Neighbor::new(2, AdjacentDirection::North);

        assert_eq!(subject.matches(&[north_one, south_one.clone()]), false);
        assert_eq!(subject.matches(&[north_two, south_one.clone()]), true);
        assert_eq!(subject.matches(&[south_one]), true);
    }
}
