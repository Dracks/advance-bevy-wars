use std::collections::{HashMap, HashSet};

use assets_helper::AssetsTrait;
use auto_tiler::{AutoTiler, BoardTrait, Neighbor};
use bevy::{prelude::*, render::render_resource::encase::private::Length};
use bevy_flair::style::components::NodeStyleSheet;
use rand::seq::IndexedRandom;
use ui_helpers::prelude::{LoadFiles, Loading, LoadingPlugin, clean_entities};

use crate::{
    GameState,
    assets::FileAssets,
    board::{
        direction::Direction,
        map::{Map, MapAssetLoader},
        samples::base_board,
        terrain::{Terrain, build_auto_tiler},
    },
};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ShowBoard;

pub struct BoardPlugin;

#[derive(SubStates, Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
#[source(ShowBoard = ShowBoard)]
pub enum BoardLoad {
    #[default]
    Loading,
    Complete,
}

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Board::random(uvec2(50, 30)))
            .insert_resource(Tiler(build_auto_tiler()))
            .add_systems(OnEnter(BoardLoad::Complete), spawn_terrain)
            .add_systems(OnExit(ShowBoard), drop_terrain);

        app.insert_resource(base_board());

        app.init_asset::<Map>()
            .init_asset_loader::<MapAssetLoader>();

        app
            .add_sub_state::<BoardLoad>()
            .add_plugins(LoadingPlugin::<BoardLoad>::new())
            .add_systems(OnEnter(ShowBoard), spawn_loading)
            .add_systems(
                OnExit(BoardLoad::Loading),
                clean_entities::<Loading<BoardLoad>>,
            );
    }
}

fn spawn_loading(mut commands: Commands, assets: Res<AssetServer>) {
    let loading = LoadFiles::from_duration(0.1)
        .with_assets(vec![FileAssets::MapTestAbwm.load::<Map>(&assets).into()]);

    commands.insert_resource(loading);
    commands.spawn((
        NodeStyleSheet::new(FileAssets::MenuStyleMenuCss.load(&assets)),
        Node::default(),
        Name::new("loading_screen"),
        Loading::new(BoardLoad::Complete),
        children![(
            Text::new("Loading..."),
            Node {
                margin: UiRect::bottom(Val::Px(30.0)),
                ..default()
            }
        ),],
    ));
}

#[derive(Component)]
pub struct MainBoard;

#[derive(Resource)]
pub struct Tiler(AutoTiler<Terrain, UVec2>);

#[derive(Resource)]
pub struct Board {
    width: usize,
    height: usize,
    terrains: Vec<Vec<Terrain>>,
    layers: Vec<BoardLayer>,
}

struct BoardLayer {
    tiles: HashMap<UVec2, Terrain>,
}

impl BoardLayer {
    fn build(
        terrains: &Vec<Vec<Terrain>>,
        required: &HashSet<Terrain>,
        fill: Option<Terrain>,
    ) -> Self {
        let mut tiles = HashMap::new();
        for (y, row) in terrains.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                let coord = uvec2(x as u32, y as u32);
                if required.contains(tile) {
                    tiles.insert(coord, tile.clone());
                } else if let Some(default) = fill {
                    tiles.insert(coord, default);
                }
            }
        }

        Self { tiles }
    }
}

impl BoardTrait<Terrain, UVec2, Direction> for BoardLayer {
    fn get(&self, pos: &UVec2) -> Option<&Terrain> {
        self.tiles.get(pos)
    }

    fn get_neighbors(
        &self,
        pos: &UVec2,
        directions: &[Direction],
    ) -> Vec<Neighbor<Terrain, Direction>> {
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
    fn new(width: usize, height: usize, terrains: Vec<Vec<Terrain>>) -> Self {
        let layers = [
            BoardLayer::build(
                &terrains,
                &HashSet::from([Terrain::Plain, Terrain::Sea, Terrain::Road, Terrain::Beach]),
                Some(Terrain::Plain),
            ),
            BoardLayer::build(
                &terrains,
                &HashSet::from([Terrain::Mountain, Terrain::Forest]),
                None,
            ),
        ];
        Self {
            width,
            height,
            terrains,
            layers: layers.into(),
        }
    }
    pub fn get_size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn fill(width: usize, height: usize, terrain: Terrain) -> Self {
        Self {
            width,
            height,
            terrains: vec![vec![terrain; width]; height],
            layers: Default::default(),
        }
    }

    pub fn get_all(&self) -> Vec<UVec2> {
        let mut all_linear = Vec::with_capacity(self.width * self.height);
        for x in 0..self.width {
            for y in 0..self.height {
                all_linear.push(uvec2(x as u32, y as u32))
            }
        }
        all_linear
    }

    pub fn random(size: UVec2) -> Self {
        let width = size.x as usize;
        let height = size.y as usize;
        let terrains = vec![
            Terrain::Mountain,
            Terrain::Plain,
            Terrain::Road,
            Terrain::Sea,
            Terrain::Beach,
            Terrain::Forest,
        ];
        let mut tiles = Vec::with_capacity(height);
        let mut rng = rand::rng();
        for _ in 0..size.y {
            let mut row = Vec::with_capacity(width);
            for _ in 0..size.x {
                match terrains.choose(&mut rng) {
                    Some(terrain) => row.push(terrain.clone()),
                    None => panic!("No terrain picked?"),
                }
            }
            tiles.push(row);
        }
        Self::new(width, height, tiles)
    }
}

impl BoardTrait<Terrain, UVec2, Direction> for Board {
    fn get(&self, pos: &UVec2) -> Option<&Terrain> {
        let x = pos.x as usize;
        let y = pos.y as usize;
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(&self.terrains[pos.y as usize][pos.x as usize])
    }

    fn get_neighbors(
        &self,
        pos: &UVec2,
        directions: &[Direction],
    ) -> Vec<Neighbor<Terrain, Direction>> {
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

impl From<Vec<Vec<&str>>> for Board {
    fn from(value: Vec<Vec<&str>>) -> Self {
        let height = value.len();
        assert!(height > 0, "The value must contain data (Height = 0)");
        let width = value[0].len();
        assert!(width > 0, "The value must contain data (Width = 0)");
        let mut tiles = Vec::with_capacity(height);
        for y in 0..height {
            let row = value[y].clone();
            assert_eq!(row.len(), width, "Row {y} has an invalid width");
            tiles.push(row.iter().map(|text| Terrain::from(*text)).collect());
        }
        bevy::log::info!("Transforming {width} {height}");
        Self::new(width, height, tiles)
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::fill(20, 20, Terrain::Plain)
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

fn spawn_terrain(
    mut commands: Commands,
    assets: Res<AssetServer>,
    auto_tiler: Res<Tiler>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    board: Res<Board>,
) {
    let helper = TileHelper::new(uvec2(68, 45));
    let map = FileAssets::MapTestAbwm.load::<Map>(&assets);
    bevy::log::info!("The Loaded map! {:?}", map);

    let texture_handle = FileAssets::ImagesGameTerrainPng.load(&assets);
    let texture_atlas = helper.atlas_layout(UVec2::splat(32));
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let auto_tiler: &AutoTiler<Terrain, UVec2> = &auto_tiler.0;
    let layers = board.layers.length();
    commands
        .spawn((Transform::IDENTITY, Visibility::Inherited, MainBoard))
        .with_children(|parent| {
            for pos in board.get_all() {
                for (idx, layer) in board.layers.iter().enumerate() {
                    if let Some(tile_coords) = auto_tiler.get_tile::<UVec2, Direction>(&*layer, pos)
                    {
                        parent.spawn((
                            Sprite::from_atlas_image(
                                texture_handle.clone(),
                                TextureAtlas {
                                    layout: texture_atlas_handle.clone(),
                                    index: helper.index(tile_coords),
                                },
                            ),
                            Transform::from_translation(vec3(
                                (pos.x * 32) as f32,
                                (pos.y * 32) as f32,
                                idx as f32 - layers as f32,
                            )),
                        ));
                    }
                }
            }
        });
}

fn drop_terrain(mut commands: Commands, query: Query<Entity, With<MainBoard>>) {
    for board in query.iter() {
        commands.entity(board).despawn();
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
