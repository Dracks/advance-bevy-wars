use crate:: AsMask;

#[derive(Clone, Debug)]
pub struct Neighbor<T: Clone, D: AsMask> {
    pub direction: D,
    pub terrain: T,
}

impl<T: Clone, D: AsMask> Neighbor<T, D> {
    pub fn new(terrain: T, direction: D) -> Self {
        Self { terrain, direction }
    }
}

pub trait BoardTrait<T: Clone, P, D: AsMask> {
    fn get(&self, pos: &P) -> Option<&T>;
    fn get_neighbors(&self, pos: &P, directions: &[D]) -> Vec<Neighbor<T, D>>;
}
