use crate::{
    board::{BoardTrait, Neighbor},
    rules::{Direction, Requirement},
};

pub struct TileDefinition<T, I> {
    tile: I,
    terrain: T,
    // first list is or, second is and
    rules: Vec<Vec<Requirement<T>>>,
    priority: i32,
}

impl<I, T: Eq + Clone> TileDefinition<T, I> {
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

    pub fn matches(&self, neighbors: &Vec<Neighbor<T>>) -> bool {
        self.rules.iter().any(|possibility| {
            possibility
                .iter()
                .all(|requirement| requirement.matches(neighbors))
        })
    }
}

#[derive(Default)]
pub struct AutoTiler<T, I> {
    tiles: Vec<TileDefinition<T, I>>,
}

impl<T: Eq + Clone, I: Copy> AutoTiler<T, I> {
    pub fn add_tile(&mut self, tile: TileDefinition<T, I>) -> &mut Self {
        self.tiles.push(tile);
        self
    }

    pub fn get_tile<P>(&self, board: &impl BoardTrait<T, P>, pos: P) -> Option<I> {
        let Some(terrain) = board.get(&pos) else {
            return None;
        };
        let neighbors = board.get_neighbors(&pos, Direction::ALL.as_slice());
        let tile = self
            .tiles
            .iter()
            .filter(|tile| tile.terrain == *terrain)
            .find(|tile| tile.matches(&neighbors));
        tile.map(|t| t.tile)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::math::{UVec2, uvec2};

    struct TestBoard {
        tiles: std::collections::HashMap<UVec2, i32>,
    }

    impl BoardTrait<i32, UVec2> for TestBoard {
        fn get(&self, pos: &UVec2) -> Option<&i32> {
            self.tiles.get(pos)
        }

        fn get_neighbors(&self, pos: &UVec2, directions: &[Direction]) -> Vec<Neighbor<i32>> {
            let neighbors = directions
                .iter()
                .filter_map(|dir| {
                    let neighbor_pos = match dir {
                        Direction::North => uvec2(pos.x, pos.y.checked_sub(1)?),
                        Direction::South => uvec2(pos.x, pos.y + 1),
                        Direction::East => uvec2(pos.x + 1, pos.y),
                        Direction::West => uvec2(pos.x.checked_sub(1)?, pos.y),
                        _ => return None,
                    };
                    self.get(&neighbor_pos)
                        .map(|terrain| Neighbor::new(*terrain, *dir))
                })
                .collect();
            println!("{:#?}", neighbors);
            neighbors
        }
    }

    fn get_tiler(terrain: i32) -> AutoTiler<i32, char> {
        let auto_tiler = AutoTiler::default();
            // Tile 'a' - cap veí específic (per defecte)
        auto_tiler.add_tile(TileDefinition::new('a', terrain))
            // Tile 'b' - requereix veí al Nord
            .add_tile(
                TileDefinition::new('b', terrain)
                    .add_possible_requirements(vec![Requirement::new(1, &vec![Direction::North])]),
            )
            // Tile 'c' - requereix veí a l'Est
            .add_tile(
                TileDefinition::new('c', terrain)
                    .add_possible_requirements(vec![Requirement::new(5, &vec![Direction::West])]),
            );
        auto_tiler;
    }

    fn create_board(grid: Vec<Vec<i32>>) -> std::collections::HashMap<UVec2, i32> {
        let mut tiles = std::collections::HashMap::new();
        for (y, row) in grid.iter().enumerate() {
            for (x, &terrain) in row.iter().enumerate() {
                tiles.insert(uvec2(x as u32, y as u32), terrain);
            }
        }
        tiles
    }

    #[test]
    fn test_1_way_handling() {
        let grid = vec![vec![0, 1, 0], vec![2, 5, 3], vec![0, 4, 0]];

        let tiles = create_board(grid);
        let board = TestBoard { tiles };

        /*
                let tiler_0 = get_tiler(0);
                let none = tiler_0.get_tile(&board, uvec2(0, 0));
                assert!(none.is_some());
                assert_eq!(none.unwrap(), 'a');
        */

        let tiler_1 = get_tiler(5);
        let n = tiler_1.get_tile(&board, uvec2(1, 1));
        assert!(n.is_some());
        assert_eq!(n.unwrap(), 'b');

        let tiler_3 = get_tiler(3);
        let e = tiler_3.get_tile(&board, uvec2(1, 2));
        assert!(e.is_some());
        assert_eq!(e.unwrap(), 'c');
    }
}
