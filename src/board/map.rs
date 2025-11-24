use std::collections::HashMap;

use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
};
use thiserror::Error;
use toml::Table;

use crate::{
    board::{Board, Direction, terrain::TileTerrain},
    interactive::{Income, Life, Movement, MovementType, Owner},
    matrix::Matrix,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Terrain {
    Plane,
    Road,
    Mountain,
    Sea,
    Beach,
    Forest,
    //River,
    //Wall,
    //BreakableWall(bool)
}

pub struct UnknownTerrain(String);

impl TryFrom<&str> for Terrain {
    type Error = UnknownTerrain;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim() {
            "p" => Ok(Terrain::Plane),
            "r" => Ok(Terrain::Road),
            "m" => Ok(Terrain::Mountain),
            //"b" => Ok(Terrain::Bridge),
            "B" => Ok(Terrain::Beach),
            //"w" => Ok(Terrain::Wall),
            "f" => Ok(Terrain::Forest),
            "s" => Ok(Terrain::Sea),
            // "b" => Ok(Terrain::Beach),
            // "rv" => Ok(Terrain::River),
            // "w" => Ok(Terrain::Wall),
            // "bw" => Ok(Terrain::BreakableWall(false)),
            // "bwd" => Ok(Terrain::BreakableWall(true)),
            value => Err(UnknownTerrain(value.into())),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Building {
    pub owner: Owner,
    pub income: Income,
    pub build_type: BuildingType,
}
#[derive(Debug, Clone, Copy)]
pub enum BuildingType {
    City,
    //Town,
    Factory,
    Headquarters,
    //Port,
    //Airport,
    //OilRig,
}

pub struct UnknownBuildingType;
impl TryFrom<&str> for BuildingType {
    type Error = UnknownBuildingType;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "headquarters" => Ok(Self::Headquarters),
            "city" => Ok(Self::City),
            "factory" => Ok(Self::Factory),
            _ => Err(UnknownBuildingType),
        }
    }
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Unit {
    pub owner: Owner,
    pub health: Life,
    pub unit_type: UnitType,
    pub movement: Movement,
}

impl Unit {
    pub fn get_movements(&self,pos:UVec2, board: &Board) -> Vec<PossibleMovement> {
        let mut movements : HashMap<UVec2, PossibleMovement> = HashMap::default();
        let mut pending_check = vec![PossibleMovement{
            cost: 0,
            layer: 0,
            position: pos,
        }];
        let total_movement = self.movement.movements;
        while let Some(to_check) = pending_check.pop() {
            let is_new_or_better = match movements.get(&to_check.position) {
                Some(existing) => existing.cost>to_check.cost,
                None => true
            };
            if is_new_or_better {
                for dir in Direction::ADJACENT {
                    bevy::log::info!("Direction: {:?}", dir);
                    let Some(new_pos) = dir.move_point(&to_check.position) else {
                        continue
                    };
                    let Some(terrain) = board.get(&new_pos) else {
                        continue;
                    };
                    let Some(move_cost) = self.movement.mov_type.cost(terrain) else {
                        continue;
                    };
                    let new_cost = to_check.cost + move_cost;
                    bevy::log::info!("New Cost: {}", new_cost);
                    if new_cost < total_movement{
                        pending_check.push(PossibleMovement { position: new_pos, layer: to_check.layer+1, cost: new_cost });
                    }
                }
                movements.insert(to_check.position, to_check);
            }
        }
        bevy::log::info!("We have possible movements! {}", movements.len());
        movements.into_iter().map(|(_, movement)| movement).collect()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UnitType {
    Infantry,
    Mech,
    Reccon,
    Tank,
}

pub struct PossibleMovement {
    pub position: UVec2,
    pub layer: u32,
    pub cost: u32
}

impl UnitType {
    pub fn get_movement(&self) -> u32 {
        match self {
            Self::Infantry => 30,
            Self::Mech => 25,
            Self::Reccon => 50,
            Self::Tank => 45,
        }
    }
}

pub struct UnknownUnitType;
impl TryFrom<&str> for UnitType {
    type Error = UnknownUnitType;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "infantry" => Ok(Self::Infantry),
            "mech" => Ok(Self::Mech),
            _ => Err(UnknownUnitType),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MapCell {
    pub terrain: Terrain,
    pub building: Option<Building>,
    pub unit: Option<Unit>,
}

impl Default for MapCell {
    fn default() -> Self {
        MapCell {
            terrain: Terrain::Plane,
            building: None,
            unit: None,
        }
    }
}

#[derive(Asset, TypePath, Debug, Clone)]
pub struct Map {
    pub cells: Matrix<MapCell>,
}

impl Map {
    pub fn empty() -> Self {
        Self {
            cells: Matrix::new(1, 1, MapCell::default()),
        }
    }
    pub fn width(&self) -> usize {
        self.cells.cols()
    }

    pub fn height(&self) -> usize {
        self.cells.rows()
    }

    pub fn get_size(&self) -> (usize, usize) {
        self.cells.size()
    }

    pub fn get(&self, idx: (usize, usize)) -> Option<&MapCell> {
        self.cells.get(idx.0, idx.1)
    }
}

impl Into<Matrix<TileTerrain>> for &Map {
    fn into(self) -> Matrix<TileTerrain> {
        let new_cells: Vec<_> = self
            .cells
            .iter()
            .map(|cell| TileTerrain::from(&cell.terrain))
            .collect();
        Matrix::from_vec(new_cells, self.cells.cols(), self.cells.rows())
            .expect("Matrix should be valid")
    }
}

// Asset loader
#[derive(Default)]
pub struct MapAssetLoader;

#[derive(Debug, Error)]
pub enum MapLoaderError {
    #[error("Could not read file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid map format: {0}")]
    ParseError(String),
}

impl AssetLoader for MapAssetLoader {
    type Asset = Map;
    type Settings = ();
    type Error = MapLoaderError;

    fn extensions(&self) -> &[&str] {
        &["abwm"]
    }

    fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext,
    ) -> impl bevy::tasks::ConditionalSendFuture<Output = std::result::Result<Self::Asset, Self::Error>>
    {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let content = std::str::from_utf8(&bytes)
                .map_err(|e| MapLoaderError::ParseError(e.to_string()))?;

            parse_map(content)
        })
    }
}

fn parse_position(key: &str) -> Option<(usize, usize)> {
    let coords: Vec<_> = key.split('x').collect();
    if coords.len() < 2 {
        None
    } else {
        let x: usize = coords[0].parse().unwrap();
        let y: usize = coords[1].parse().unwrap();
        if x == 0 || y == 0 {
            None
        } else {
            Some((x - 1, y - 1))
        }
    }
}

fn parse_v1_building(building_source: &Table) -> Result<Building, MapLoaderError> {
    let owner_id = building_source
        .get("owner")
        .map(|d| d.as_integer())
        .flatten()
        .unwrap_or(0);

    if owner_id < 0 {
        return Err(MapLoaderError::ParseError(
            "Owner ID must be a positive number".into(),
        ));
    }
    let Some(building_type) = building_source.get("type").map(|d| d.as_str()).flatten() else {
        return Err(MapLoaderError::ParseError(
            "Building type must be specified".into(),
        ));
    };

    let build_type = BuildingType::try_from(building_type).map_err(|_err| {
        MapLoaderError::ParseError(format!("Invalid building type: {building_type}"))
    })?;

    Ok(Building {
        build_type,
        owner: Owner(owner_id as u8),
        income: Income(1000),
    })
}
fn parse_v1_unit(unit_source: &Table) -> Result<Unit, MapLoaderError> {
    let Some(owner_id) = unit_source.get("owner").map(|d| d.as_integer()).flatten() else {
        return Err(MapLoaderError::ParseError(
            "Owner ID is required and should be a positive number".into(),
        ));
    };
    if owner_id < 1 {
        return Err(MapLoaderError::ParseError(
            "Owner Id must be equals or greater than 1 for units".into(),
        ));
    };

    let Some(unit_type) = unit_source.get("type").map(|d| d.as_str()).flatten() else {
        return Err(MapLoaderError::ParseError(
            "Unit type must be specified".into(),
        ));
    };
    let unit_type = UnitType::try_from(unit_type)
        .map_err(|_err| MapLoaderError::ParseError(format!("Invalid unit type: {unit_type}")))?;

    let health = unit_source
        .get("life")
        .map(|d| d.as_integer())
        .flatten()
        .unwrap_or(100);
    if health < 1 {
        return Err(MapLoaderError::ParseError(
            "Life of a unit must be greater than 0".into(),
        ));
    }

    Ok(Unit {
        owner: Owner(owner_id as u8),
        health: Life(health as u8),
        unit_type,
        movement: Movement { mov_type: MovementType::Foot, movements: 40 }
    })
}
fn parse_v1(map_source: &Table) -> Result<Map, MapLoaderError> {
    let Some(width) = map_source.get("width").map(|d| d.as_integer()).flatten() else {
        return Err(MapLoaderError::ParseError(
            "Missing width or is not an integer".into(),
        ));
    };

    let Some(height) = map_source.get("height").map(|d| d.as_integer()).flatten() else {
        return Err(MapLoaderError::ParseError(
            "Missing height or is not an integer".into(),
        ));
    };
    let width = width as usize;
    let height = height as usize;

    let mut map = Matrix::new(width, height, MapCell::default());

    let Some(terrain) = map_source.get("terrain").map(|d| d.as_array()).flatten() else {
        return Err(MapLoaderError::ParseError(
            "Missing terrain or is not an array".into(),
        ));
    };
    if terrain.len() != height {
        return Err(MapLoaderError::ParseError(format!(
            "Invalid terrain height {}, it doesn't match the property height {height}",
            terrain.len()
        )));
    }
    for idy in 0..height {
        let Some(terrain_row) = terrain[idy].as_array() else {
            return Err(MapLoaderError::ParseError(format!(
                "Invalid terrain, row {idy} is not an array"
            )));
        };
        if terrain_row.len() != width {
            return Err(MapLoaderError::ParseError(format!(
                "Invalid terrain width {} at row {idy}, it doesn't match the property width {width}",
                terrain_row.len()
            )));
        }
        for idx in 0..width {
            let Some(cell) = terrain_row[idx].as_str() else {
                return Err(MapLoaderError::ParseError(format!(
                    "Invalid terrain at ({idx}, {idy})"
                )));
            };
            let terrain_cell = Terrain::try_from(cell).map_err(|err| {
                MapLoaderError::ParseError(format!("Unknown terrain type {}", err.0))
            })?;
            map[(idx, idy)] = MapCell {
                terrain: terrain_cell,
                building: None,
                unit: None,
            };
        }
    }

    let empty_list = Table::new();
    let units = map_source
        .get("units")
        .map(|unit| unit.as_table())
        .flatten()
        .unwrap_or(&empty_list);
    for (key, value) in units.iter() {
        let Some(coords) = parse_position(key) else {
            return Err(MapLoaderError::ParseError(format!(
                "Invalid coord in units: {key}"
            )));
        };
        if coords.0 > width || coords.1 > height {
            return Err(MapLoaderError::ParseError(format!(
                "Invalid coords {:?}, they are bigger than ({},{})",
                coords, width, height
            )));
        }
        let Some(unit_data) = value.as_table() else {
            return Err(MapLoaderError::ParseError(format!(
                "Invalid contents in unit coords {:?}",
                coords
            )));
        };
        map[coords].unit = Some(parse_v1_unit(unit_data)?)
    }

    let empty_list = Table::new();
    let buildings = map_source
        .get("buildings")
        .map(|building| building.as_table())
        .flatten()
        .unwrap_or(&empty_list);
    for (key, value) in buildings.iter() {
        let Some(coords) = parse_position(key) else {
            return Err(MapLoaderError::ParseError(format!(
                "Invalid coord in build: {key}"
            )));
        };
        if coords.0 > width || coords.1 > height {
            return Err(MapLoaderError::ParseError(format!(
                "Invalid coords {:?}, they are bigger than ({},{})",
                coords, width, height
            )));
        }
        let Some(building_data) = value.as_table() else {
            return Err(MapLoaderError::ParseError(format!(
                "Invalid contents in build coords {:?}",
                coords
            )));
        };
        map[coords].building = Some(parse_v1_building(building_data)?)
    }

    Ok(Map { cells: map })
}

fn parse_map(content: &str) -> Result<Map, MapLoaderError> {
    let raw_file: Table = toml::from_str(content).map_err(|err| {
        bevy::log::error!("Error loading map {err}");
        MapLoaderError::ParseError("Invalid file format".into())
    })?;
    let version = raw_file.get("version").map_or_else(
        || Err(MapLoaderError::ParseError("Version not found".into())),
        |data| Ok(data.as_integer()),
    )?;

    match version {
        Some(1) => parse_v1(&raw_file),
        _ => Err(MapLoaderError::ParseError(
            format!("Unsupported map version {:?}", version).into(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_map() {
        let data = "version=1
        width=1
        height=1
        terrain=[[\"p\"]]";

        let res = parse_map(data);
        println!("return {:?}", res);
        assert!(res.is_ok());
        assert_eq!(res.unwrap().cells[(0, 0)].terrain, Terrain::Plane);
    }

    #[test]
    fn test_map_with_units() {
        let data = "version=1
            width=1
            height=2
            terrain = [
                [\"p\"],
                [\"p\"],
            ]
            [units]
            1x1 = {type=\"infantry\", owner=1}
            1x2 = {type=\"Mech\", owner=2, life=50}
            ";

        let map = parse_map(data);
        println!("Map: {:?}", map);
        assert!(map.is_ok());
        let map = map.unwrap();
        assert_eq!(
            map.cells[(0, 0)].unit,
            Some(Unit {
                owner: Owner(1),
                health: Life(100),
                unit_type: UnitType::Infantry,
                movement: Movement{
                    mov_type: MovementType::Foot,
                    movements: 30,
                }
            })
        );
        assert_eq!(
            map.cells[(0, 1)].unit,
            Some(Unit {
                owner: Owner(2),
                health: Life(50),
                unit_type: UnitType::Mech,
                movement: Movement {
                    mov_type: MovementType::Foot,
                    movements: 30,
                }
            })
        );
    }
}
