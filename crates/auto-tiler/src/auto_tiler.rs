use std::hash::Hash;

use crate::{
    board::{BoardTrait, Neighbor},
    rules::{AsMask, Requirement},
};

pub struct TileDefinition<T, I> {
    tile: I,
    terrain: T,
    // first list is or, second is and
    rules: Vec<Vec<Requirement<T>>>,
    priority: i32,
}

impl<I: std::fmt::Debug, T: Eq + Clone + std::fmt::Debug + Hash> TileDefinition<T, I> {
    pub fn new(tile: I, terrain: T) -> Self {
        Self {
            tile,
            terrain,
            rules: vec![],
            priority: 0,
        }
    }

    pub fn add_possible_requirements(mut self, requirements: Vec<Requirement<T>>) -> Self {
        self.rules.push(requirements);
        self
    }

    pub fn change_priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }

    pub fn matches<D: AsMask>(&self, neighbors: &Vec<Neighbor<T, D>>) -> bool {
        if self.rules.len() == 0 {
            return true;
        }
        let result = self.rules.iter().any(|possibility| {
            possibility
                .iter()
                .all(|requirement| requirement.matches(neighbors))
        });
        // println!("Checking matches {:?} vs {:#?} = {result}", self.terrain, neighbors);
        result
    }
}

#[derive(Default)]
pub struct AutoTiler<T, I> {
    tiles: Vec<TileDefinition<T, I>>,
}

impl<T: Eq + Clone + std::fmt::Debug + Hash, I: Copy + std::fmt::Debug> AutoTiler<T, I> {
    pub fn add_tile(&mut self, tile: TileDefinition<T, I>) -> &mut Self {
        let insert_at = match self
            .tiles
            .binary_search_by(|e| e.priority.cmp(&tile.priority))
        {
            Ok(insert_at) | Err(insert_at) => insert_at,
        };
        self.tiles.insert(insert_at, tile);
        self
    }

    pub fn get_tile<P, D: AsMask>(&self, board: &impl BoardTrait<T, P, D>, pos: P) -> Option<I> {
        let Some(terrain) = board.get(&pos) else {
            return None;
        };
        let neighbors = board.get_neighbors(&pos, D::ALL.iter().as_slice());
        let tile = self
            .tiles
            .iter()
            .filter(|tile| tile.terrain == *terrain)
            .find(|tile| tile.matches(&neighbors));
        tile.map(|t| t.tile)
    }

    pub fn get_defined_tiles(&self) -> Vec<I> {
        self.tiles.iter().map(|definition| definition.tile).collect()
    }
}
