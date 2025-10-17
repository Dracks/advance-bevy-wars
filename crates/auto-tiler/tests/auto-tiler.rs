
use ::auto_tiler::*;

use bevy::math::{UVec2, uvec2};

struct TestBoard {
    tiles: std::collections::HashMap<UVec2, i32>,
}

impl TestBoard {
    fn from_grid(grid: Vec<Vec<i32>>) -> Self {
        let mut tiles = std::collections::HashMap::new();
        for (y, row) in grid.iter().enumerate() {
            for (x, &terrain) in row.iter().enumerate() {
                tiles.insert(uvec2(x as u32, y as u32), terrain);
            }
        }
        Self { tiles }
    }
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
        // println!("{:#?}", neighbors);
        neighbors
    }
}

fn get_tiler(terrain: i32) -> AutoTiler<i32, char> {
    let mut auto_tiler = AutoTiler::default();
    // Tile 'a' - cap veí específic (per defecte)
    auto_tiler
        .add_tile(TileDefinition::new('a', terrain).change_priority(-1))
        // Tile 'b' - requereix veí al Nord
        .add_tile(
            TileDefinition::new('b', terrain)
                .add_possible_requirements(vec![Requirement::new(4, &vec![Direction::South])]),
        )
        // Tile 'c' - requereix veí a l'Est
        .add_tile(
            TileDefinition::new('c', terrain)
                .add_possible_requirements(vec![Requirement::new(5, &vec![Direction::West])]),
        );
    auto_tiler
}

#[test]
fn test_a_on_all_terrains() {

    #[rustfmt::skip]
    let grid = vec![
        vec![1, 1, 1],
        vec![1, 1, 1],
        vec![1, 1, 1]
    ];

    let board = TestBoard::from_grid(grid);

    let tiler = get_tiler(1);

    let mut result : Vec<(UVec2, Option<char>)> = Vec::default();
    for coord in board.tiles.keys() {
        result.push((coord.clone(), tiler.get_tile(&board, coord.clone())));
    }

    println!("Result: {:?}", result);
    let count = result.iter().filter(|(_, tile)| tile.is_some()).count();
    assert_eq!(count, 9);
}

#[test]
fn test_1_way_handling() {
    #[rustfmt::skip]
    let grid = vec![
        vec![0, 1, 0],
        vec![2, 5, 3],
        vec![0, 4, 0]
    ];

    let board = TestBoard::from_grid(grid);


    let tiler_0 = get_tiler(0);
    let none = tiler_0.get_tile(&board, uvec2(0, 0));
    assert!(none.is_some());
    assert_eq!(none.unwrap(), 'a');


    let tiler_1 = get_tiler(5);
    let n = tiler_1.get_tile(&board, uvec2(1, 1));
    assert!(n.is_some());
    assert_eq!(n.unwrap(), 'b');

    let tiler_3 = get_tiler(3);
    let e = tiler_3.get_tile(&board, uvec2(1, 2));
    assert!(e.is_some());
    assert_eq!(e.unwrap(), 'c');
}
