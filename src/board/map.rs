use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
};
use thiserror::Error;
use toml::Table;

use crate::interactive::{Income, Life, Owner};

#[derive(Debug, PartialEq)]
pub enum Terrain {
    Plane,
    Road,
    Mountain,
    // Sea,
    //Beach,
    //River,
    //Wall,
    //BreakableWall(bool)
}
#[derive(Debug)]
pub struct Building {
    owner: Owner,
    income: Income,
    build_type: BuildingType,
}
#[derive(Debug)]
pub enum BuildingType {
    City,
    //Town,
    Factory,
    Headquarters,
    //Port,
    //Airport,
    //OilRig,
}
#[derive(Debug, PartialEq)]
pub struct Unit {
    owner: Owner,
    health: Life,
    unit_type: UnitType,
}
#[derive(Debug, PartialEq)]
pub enum UnitType {
    Infantry,
    Mech,
    Reccon,
    Tank,
}

pub struct UnknownUnitType;
impl TryFrom<&str> for UnitType {
    type Error = UnknownUnitType;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "infantry" => Ok(Self::Infantry),
            "mech" => Ok(Self::Mech),
            _ => Err(UnknownUnitType)
        }
    }
}

#[derive(Debug)]
pub struct MapCell {
    terrain: Terrain,
    building: Option<Building>,
    unit: Option<Unit>,
}

#[derive(Asset, TypePath, Debug)]
pub struct Map {
    cells: Vec<Vec<MapCell>>,
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
    let coords : Vec<_> = key.split('x').collect();
    if coords.len()<2 {
        None
    } else {
        let x : usize = coords[0].parse().unwrap();
        let y : usize = coords[1].parse().unwrap();
        if x==0 || y ==0 {
            None
        } else {
        Some((x-1, y-1))
        }
    }
}
fn parse_v1_unit(unit_source: &Table) -> Result<Unit, MapLoaderError> {
    let Some(owner_id) = unit_source.get("owner").map(|d| d.as_integer()).flatten() else {
        return Err(MapLoaderError::ParseError("Owner ID is required and should be a positive number".into()));
    };
    if owner_id <1 {
        return Err(MapLoaderError::ParseError("Owner Id must be equals or greater than 1 for units".into()));
    };

    let Some(unit_type) = unit_source.get("type").map(|d| d.as_str()).flatten() else {
        return Err(MapLoaderError::ParseError("Unit type must be specified".into()));
    };
    let unit_type = UnitType::try_from(unit_type).map_err(|err| MapLoaderError::ParseError(format!("Invalid unit type: {unit_type}")))?;

    let health = unit_source.get("life").map(|d| d.as_integer()).flatten().unwrap_or(100);
    if health <1 {
        return Err(MapLoaderError::ParseError("Life of a unit must be greater than 0".into()));
    }

    Ok(Unit {
        owner: Owner(owner_id as u8),
        health: Life(health as u8),
        unit_type,
    })

}
fn parse_v1(map_source: &Table) -> Result<Map, MapLoaderError> {
    let Some(width) = map_source.get("width").map(|d| d.as_integer()).flatten() else {
        return Err(MapLoaderError::ParseError("Missing width or is not an integer".into()))
    };

    let Some(height) = map_source.get("height").map(|d| d.as_integer()).flatten() else {
        return Err(MapLoaderError::ParseError("Missing height or is not an integer".into()))
    };
    let width = width as usize;
    let height = height as usize;

    let mut columns = Vec::new();
    columns.reserve(height as usize);

    let Some(terrain) = map_source.get("terrain").map(|d| d.as_array()).flatten() else {
        return Err(MapLoaderError::ParseError("Missing terrain or is not an array".into()))
    };
    if terrain.len() != height {
        return Err(MapLoaderError::ParseError("Invalid terrain height, it doesn't match the property height".into()))
    }
    for idy in 0..height{
        let Some(terrain_row) = terrain[idy].as_array() else {
            return Err(MapLoaderError::ParseError(format!("Invalid terrain, row at column {idy} is not an array")));
        };
        if terrain_row.len() != width {
            return Err(MapLoaderError::ParseError(format!("Invalid terrain width at column {idy}, it doesn't match the property width")));
        }
        let mut row = Vec::new();
        row.reserve(width as usize);
        for idx in 0..width {
            let Some(cell) = terrain_row[idx].as_str() else {
                return Err(MapLoaderError::ParseError(format!("Invalid terrain at ({idx}, {idy})")));
            };
            let terrain_cell = parse_terrain(cell)?;
            row.push( MapCell {
                terrain: terrain_cell,
                building: None,
                unit: None,
            });
        }
        columns.push(row);
    }
    let mut map = Map{cells: columns};

    let empty_list = Table::new();
    let units = map_source.get("units").map(|units| units.as_table()).flatten().unwrap_or(&empty_list);
    for (key, value) in units.iter() {
        let Some(coords) = parse_position(key) else {
            return Err(MapLoaderError::ParseError(format!("Invalid coord in units: {key}")));
        };
        if coords.0>width || coords.1>height {
            return Err(MapLoaderError::ParseError(format!("Invalid coords {:?}, they are bigger than ({},{})", coords, width, height)))
        }
        let Some(unit_data) = value.as_table() else {
            return Err(MapLoaderError::ParseError(format!("Invalid contents in units coords {:?}", coords)));
        };
        map.cells[coords.1][coords.0].unit = Some(parse_v1_unit(unit_data)?)
    }


    Ok(map)
}

fn parse_map(content: &str) -> Result<Map, MapLoaderError> {
    let raw_file: Table = toml::from_str(content).map_err(|err| {
        bevy::log::error!("Error loading map {err}");
        MapLoaderError::ParseError("Invalid file format".into())
    })?;
    let version = raw_file.get("version").map_or_else(|| Err(MapLoaderError::ParseError("Version not found".into())), |data| Ok(data.as_integer()))?;

    match version {
        Some(1) => parse_v1(&raw_file),
        _ => Err(MapLoaderError::ParseError(format!("Unsupported map version {:?}", version).into()))
    }
}

fn parse_terrain(s: &str) -> Result<Terrain, MapLoaderError> {
    match s.trim() {
        "p" => Ok(Terrain::Plane),
        "r" => Ok(Terrain::Road),
        "m" => Ok(Terrain::Mountain),
        // "s" => Ok(Terrain::Sea),
        // "b" => Ok(Terrain::Beach),
        // "rv" => Ok(Terrain::River),
        // "w" => Ok(Terrain::Wall),
        // "bw" => Ok(Terrain::BreakableWall(false)),
        // "bwd" => Ok(Terrain::BreakableWall(true)),
        _ => Err(MapLoaderError::ParseError(format!(
            "Unknown terrain: {}",
            s
        ))),
    }
}
/*
fn parse_building(s: &str) -> Result<Building, MapLoaderError> {
    let parts: Vec<&str> = s.trim().split_whitespace().collect();

    if parts.is_empty() {
        return Err(MapLoaderError::ParseError("Empty building".to_string()));
    }

    let build_type = match parts[0] {
        "c" => BuildingType::City,
        //"t" => BuildingType::Town,
        "f" => BuildingType::Factory,
        "h" => BuildingType::Headquarters,
        //"pt" => BuildingType::Port,
        //"a" => BuildingType::Airport,
        //"o" => BuildingType::OilRig,
        _ => {
            return Err(MapLoaderError::ParseError(format!(
                "Unknown building type: {}",
                parts[0]
            )))
        }
    };

    let owner = if parts.len() > 1 {
        let ownerId = parts[1].parse::<u8>();
        match
        Owner(parts[1].parse().map_err(|_| {
            MapLoaderError::ParseError(format!("Invalid owner: {}", parts[1]))
        })?)
    } else {
        Owner(0) // Neutral
    };

    Ok(Building {
        owner,
        income: Income(1000), // Default income
        build_type,
    })
}

fn parse_unit(s: &str) -> Result<Unit, MapLoaderError> {
    let parts: Vec<&str> = s.trim().split_whitespace().collect();

    if parts.is_empty() {
        return Err(MapLoaderError::ParseError("Empty unit".to_string()));
    }

    let unit_type = match parts[0] {
        "i" => UnitType::Infantry,
        "r" => UnitType::Reccon,
        "me" => UnitType::Mech,
        "ta" => UnitType::Tank,
        //"ar" => UnitType::Artillery,
        _ => {
            return Err(MapLoaderError::ParseError(format!(
                "Unknown unit type: {}",
                parts[0]
            )))
        }
    };

    let owner = if parts.len() > 1 {
        Owner(parts[1].parse().map_err(|_| {
            MapLoaderError::ParseError(format!("Invalid owner: {}", parts[1]))
        })?)
    } else {
        // No Valid
        Owner(0)
    };

    let health = if parts.len() > 2 {
        Life(parts[2].parse().map_err(|_| {
            MapLoaderError::ParseError(format!("Invalid health: {}", parts[2]))
        })?)
    } else {
        Life::new()
    };

    Ok(Unit {
        owner,
        health,
        unit_type,
    })
}*/


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
        assert_eq!(res.unwrap().cells[0][0].terrain, Terrain::Plane);
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
        assert_eq!(map.cells[0][0].unit, Some(Unit{owner: Owner(1), health: Life(100), unit_type: UnitType::Infantry}));
        assert_eq!(map.cells[1][0].unit, Some(Unit{owner: Owner(2), health: Life(50), unit_type: UnitType::Mech}));
    }
}
