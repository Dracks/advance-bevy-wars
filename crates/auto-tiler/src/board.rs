use crate::rules::Direction;

#[derive(Clone, Debug)]
pub struct Neighbor<T: Clone> {
    pub direction: Direction,
    pub terrain: T,
}

impl<T: Clone> Neighbor<T> {
    pub fn new(terrain: T, direction: Direction) -> Self {
        Self { terrain, direction }
    }
}

pub trait BoardTrait<T: Clone, P> {
    fn get(&self, pos: &P) -> Option<&T>;
    fn get_neighbors(&self, pos: &P, directions: &[Direction]) -> Vec<Neighbor<T>>;
}
