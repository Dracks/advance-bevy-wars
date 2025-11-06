use std::{
    cell,
    collections::{HashMap, HashSet},
};

use assets_helper::AssetsTrait;
use auto_tiler::{AutoTiler, BoardTrait, Neighbor};
use bevy::{prelude::*, render::render_resource::encase::private::Length};

use crate::{
    assets::FileAssets,
    board::{
        direction::Direction,
        map::{Map, Terrain},
        terrain::TileTerrain,
    },
    interactive::BoardPos,
    matrix::Matrix,
};

#[derive(Component)]
pub struct MainBoard;

#[derive(Resource)]
pub struct Tiler(pub AutoTiler<TileTerrain, UVec2>);

#[derive(Resource)]
pub struct Board {
    map: Map,
    layers: Vec<BoardLayer>,
    buildings: Matrix<Option<Entity>>,
    units: Matrix<Option<Entity>>,
}

impl Default for Board {
    fn default() -> Self {
        Self::new(Map::empty())
    }
}

struct BoardLayer {
    tiles: HashMap<UVec2, TileTerrain>,
}

impl BoardLayer {
    fn build(
        map: &Matrix<TileTerrain>,
        required: &HashSet<TileTerrain>,
        fill: Option<TileTerrain>,
    ) -> Self {
        let mut tiles = HashMap::new();
        for (x, y) in map.keys() {
            let coord = uvec2(x as u32, y as u32);
            let tile = map[(x, y)];
            if required.contains(&tile) {
                tiles.insert(coord, tile.clone());
            } else if let Some(default) = fill {
                tiles.insert(coord, default);
            }
        }

        Self { tiles }
    }
}

impl BoardTrait<TileTerrain, UVec2, Direction> for BoardLayer {
    fn get(&self, pos: &UVec2) -> Option<&TileTerrain> {
        self.tiles.get(pos)
    }

    fn get_neighbors(
        &self,
        pos: &UVec2,
        directions: &[Direction],
    ) -> Vec<Neighbor<TileTerrain, Direction>> {
        directions
            .iter()
            .filter_map(|dir| Some((dir, dir.move_point(pos)?)))
            .filter_map(|(dir, neighbor_pos)| {
                self.get(&neighbor_pos)
                    .map(|terrain| Neighbor::new(*terrain, *dir))
            })
            .collect()
    }
}

impl Board {
    fn new(map: Map) -> Self {
        let layers = {
            let tiles: Matrix<TileTerrain> = (&map).into();
            [
                BoardLayer::build(
                    &tiles,
                    &HashSet::from([
                        TileTerrain::Plain,
                        TileTerrain::Sea,
                        TileTerrain::Road,
                        TileTerrain::Beach,
                    ]),
                    Some(TileTerrain::Plain),
                ),
                BoardLayer::build(
                    &tiles,
                    &HashSet::from([TileTerrain::Mountain, TileTerrain::Forest]),
                    None,
                ),
            ]
        };
        let width = map.width();
        let height = map.height();

        Self {
            map,
            layers: layers.into(),
            buildings: Matrix::default(width, height),
            units: Matrix::default(width, height),
        }
    }
    pub fn get_size(&self) -> (usize, usize) {
        self.map.get_size()
    }

    pub fn get(&self, pos: &UVec2) -> Option<&Terrain> {
        let x = pos.x as usize;
        let y = pos.y as usize;
        if x >= self.map.width() || y >= self.map.height() {
            return None;
        }
        self.map.get((x, y)).map(|cell| &cell.terrain)
    }

    pub fn spawn_terrain(
        mut commands: Commands,
        assets: Res<AssetServer>,
        maps: Res<Assets<Map>>,
        auto_tiler: Res<Tiler>,
        mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    ) {
        let helper = TileHelper::new(uvec2(68, 45));
        let unit_helper = TileHelper::new(uvec2(5, 8));
        let map_handler = FileAssets::MapTestAbwm.load::<Map>(&assets);
        let map = maps.get(&map_handler);
        let Some(map) = map else {
            bevy::log::error!("Map not correctly loaded");
            return;
        };
        let board = Board::new(map.clone());

        let texture_handle = FileAssets::ImagesGameTerrainPng.load(&assets);
        let texture_atlas = helper.atlas_layout(UVec2::splat(32));
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let auto_tiler: &AutoTiler<TileTerrain, UVec2> = &auto_tiler.0;
        let layers = board.layers.length();
        let unit_handle = FileAssets::ImagesGameUnitsInfantryPng.load(&assets);
        let unit_texture_atlas = unit_helper.atlas_layout(UVec2::splat(32));
        let unit_texture_atlas_handle = texture_atlases.add(unit_texture_atlas);

        commands
            .spawn((Transform::IDENTITY, Visibility::Inherited, MainBoard))
            .with_children(|parent| {
                for pos in board.map.cells.keys() {
                    let cell_info = &board.map.cells[pos];
                    let board_position = BoardPos::from(pos);
                    for (idx, layer) in board.layers.iter().enumerate() {
                        if let Some(tile_coords) = auto_tiler.get_tile::<UVec2, Direction>(
                            &*layer,
                            (pos.0 as u32, pos.1 as u32).into(),
                        ) {
                            parent.spawn((
                                Sprite::from_atlas_image(
                                    texture_handle.clone(),
                                    TextureAtlas {
                                        layout: texture_atlas_handle.clone(),
                                        index: helper.index(tile_coords),
                                    },
                                ),
                                Transform::from_translation(
                                    board_position.get_screen_pos(idx as i32 - layers as i32),
                                ),
                            ));
                        }
                    }
                    if let Some(unit) = cell_info.unit {
                        bevy::log::info!("We have units! {:?}", unit);
                        parent.spawn((
                            board_position.clone(),
                            Sprite::from_atlas_image(
                                unit_handle.clone(),
                                TextureAtlas {
                                    layout: unit_texture_atlas_handle.clone(),
                                    index: 0,
                                },
                            ),
                            Transform::from_translation(board_position.get_screen_pos(1)),
                        ));
                    }
                    if let Some(building) = cell_info.building {
                        bevy::log::info!("We have buildings! {:?}", building);
                        parent.spawn((
                            Sprite::from_atlas_image(
                                texture_handle.clone(),
                                TextureAtlas {
                                    layout: texture_atlas_handle.clone(),
                                    index: helper.index(uvec2(0, 37)),
                                },
                            ),
                            Transform::from_translation(board_position.get_screen_pos(0)),
                        ));
                    }
                }
            });
        commands.insert_resource(board);
    }
}

struct TileHelper {
    size: UVec2,
}

impl TileHelper {
    pub fn new(size: UVec2) -> Self {
        Self { size }
    }

    pub fn index(&self, pos: UVec2) -> usize {
        assert!(self.size.x >= pos.x);
        assert!(self.size.y >= pos.y);
        return (pos.y * self.size.x + pos.x) as usize;
    }

    pub fn atlas_layout(&self, tile_size: UVec2) -> TextureAtlasLayout {
        TextureAtlasLayout::from_grid(tile_size, self.size.x, self.size.y, None, None)
    }
}

pub fn drop_terrain(mut commands: Commands, query: Query<Entity, With<MainBoard>>) {
    for board in query.iter() {
        commands.entity(board).despawn();
    }
}

pub fn center_camera(mut camera: Query<&mut Transform, With<Camera>>, board: Res<Board>) {
    let board_size = board.get_size();
    let center = vec3(board_size.0 as f32, board_size.1 as f32, 0.) * (32. / 2.);
    for mut camera in camera.iter_mut() {
        camera.translation = center.clone()
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
}
