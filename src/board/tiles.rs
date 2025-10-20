use crate::board::terrain::Terrain;
use ::auto_tiler::*;
use bevy::prelude::*;
/*
fn add_wall_tiles(auto_tiler: &mut AutoTiler<Terrain, UVec2>, terrain: Terrain, offset: UVec2) {
    auto_tiler
        .add_tile(
            TileDefinition::new_single_terrain(uvec2(0, 0) + offset, terrain).add_possible_requirements(vec![
                Requirement::new(
                    terrain,
                    &vec![Direction::East, Direction::South, Direction::SouthEast],
                ).not_wanted_adj(),
            ]),
        )
        .add_tile(
            TileDefinition::new_single_terrain(uvec2(1, 0) + offset, terrain).add_possible_requirements(vec![
                Requirement::new(terrain, &vec![Direction::South]).not_wanted_adj(),
            ]),
        )
        .add_tile(
            TileDefinition::new_single_terrain(uvec2(2, 0) + offset, terrain).add_possible_requirements(vec![
                Requirement::new(
                    terrain,
                    &vec![Direction::West, Direction::South, Direction::SouthWest],
                ).not_wanted_adj(),
            ]),
        )
        .add_tile(
            TileDefinition::new_single_terrain(uvec2(0, 1) + offset, terrain).add_possible_requirements(vec![
                Requirement::new(terrain, &vec![Direction::East]).not_wanted_adj(),
            ]),
        )
        .add_tile(
            TileDefinition::new_single_terrain(uvec2(1, 1) + offset, terrain).add_possible_requirements(vec![
                Requirement::new(terrain, &vec![]).not_wanted_adj(),
            ]),
        )
        .add_tile(
            TileDefinition::new_single_terrain(uvec2(2, 1) + offset, terrain).add_possible_requirements(vec![
                Requirement::new(terrain, &vec![Direction::West]).not_wanted_adj(),
            ]),
        )
        .add_tile(
            TileDefinition::new_single_terrain(uvec2(0, 2) + offset, terrain).add_possible_requirements(vec![
                Requirement::new(
                    terrain,
                    &vec![Direction::East, Direction::North, Direction::NorthEast],
                ).not_wanted_adj(),
            ]),
        )
        .add_tile(
            TileDefinition::new_single_terrain(uvec2(1, 2) + offset, terrain).add_possible_requirements(vec![
                Requirement::new(terrain, &vec![Direction::North]).not_wanted_adj(),
            ]),
        )
        .add_tile(
            TileDefinition::new_single_terrain(uvec2(2, 2) + offset, terrain).add_possible_requirements(vec![
                Requirement::new(
                    terrain,
                    &vec![Direction::West, Direction::North, Direction::NorthWest],
                ).not_wanted_adj(),
            ]),
        );
}

fn add_mountain_tiles(auto_tiler: &mut AutoTiler<Terrain, UVec2>, terrain: Terrain, offset: UVec2) {
    auto_tiler
        .add_tile(
            TileDefinition::new_single_terrain(uvec2(0, 0) + offset, terrain)
                .add_possible_requirements(vec![Requirement::new(terrain, &vec![]).not_wanted_adj()]),
        )
        .add_tile(
            TileDefinition::new_single_terrain(uvec2(1, 0) + offset, terrain)
                .add_possible_requirements(vec![Requirement::new(terrain, &vec![]).not_wanted_adj()]),
        )
        .add_tile(
            TileDefinition::new_single_terrain(uvec2(0, 1) + offset, terrain)
                .add_possible_requirements(vec![Requirement::new(terrain, &vec![]).not_wanted_adj()]),
        )
        .add_tile(
            TileDefinition::new_single_terrain(uvec2(1, 1) + offset, terrain)
                .add_possible_requirements(vec![Requirement::new(terrain, &vec![]).not_wanted_adj()]),
        )
        .add_tile(
            TileDefinition::new_single_terrain(uvec2(0, 2) + offset, terrain)
                .add_possible_requirements(vec![Requirement::new(terrain, &vec![]).not_wanted_adj()]),
        )
        .add_tile(
            TileDefinition::new_single_terrain(uvec2(1, 2) + offset, terrain)
                .add_possible_requirements(vec![Requirement::new(terrain, &vec![]).not_wanted_adj()]),
        )
        .add_tile(
            TileDefinition::new_single_terrain(uvec2(2, 2) + offset, terrain)
                .add_possible_requirements(vec![Requirement::new(terrain, &vec![]).not_wanted_adj()]),
        );
}
fn add_forest_tiles(auto_tiler: &mut AutoTiler<Terrain, UVec2>, terrain: Terrain, offset: UVec2) {
    auto_tiler
        .add_tile(
            TileDefinition::new_single_terrain(uvec2(0, 0) + offset, terrain)
                .add_possible_requirements(vec![Requirement::new(terrain, &vec![]).not_wanted_adj()]),
        )
        .add_tile(
            TileDefinition::new_single_terrain(uvec2(1, 0) + offset, terrain)
                .add_possible_requirements(vec![Requirement::new(terrain, &vec![]).not_wanted_adj()]),
        )
        .add_tile(
            TileDefinition::new_single_terrain(uvec2(2, 0) + offset, terrain)
                .add_possible_requirements(vec![Requirement::new(terrain, &vec![]).not_wanted_adj()]),
        )
        .add_tile(
            TileDefinition::new_single_terrain(uvec2(0, 1) + offset, terrain)
                .add_possible_requirements(vec![Requirement::new(terrain, &vec![]).not_wanted_adj()]),
        )
        .add_tile(
            TileDefinition::new_single_terrain(uvec2(1, 1) + offset, terrain)
                .add_possible_requirements(vec![Requirement::new(terrain, &vec![]).not_wanted_adj()]),
        )
        .add_tile(
            TileDefinition::new_single_terrain(uvec2(2, 1) + offset, terrain)
                .add_possible_requirements(vec![Requirement::new(terrain, &vec![]).not_wanted_adj()]),
        );
}
*/
