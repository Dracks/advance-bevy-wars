use assets_helper::AssetsTrait;
use bevy::prelude::*;

mod board;
mod direction;
mod map;
mod terrain;

use bevy_flair::style::components::NodeStyleSheet;
pub use board::Board;
use ui_helpers::prelude::*;

use crate::{
    assets::FileAssets,
    board::{
        board::{Tiler, center_camera, drop_terrain},
        map::{Map, MapAssetLoader},
        terrain::build_auto_tiler,
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
        app.insert_resource(Board::default())
            .insert_resource(Tiler(build_auto_tiler()))
            .add_systems(
                OnEnter(BoardLoad::Complete),
                (Board::spawn_terrain, center_camera).chain(),
            )
            .add_systems(OnExit(ShowBoard), drop_terrain);

        // app.insert_resource(base_board());

        app.init_asset::<Map>()
            .init_asset_loader::<MapAssetLoader>();

        app.add_sub_state::<BoardLoad>()
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
