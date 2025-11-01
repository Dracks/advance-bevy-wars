use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
};
use thiserror::Error;
use toml::Table;

use crate::interactive::{Capturable, Income, Life, Owner};

#[derive(Debug)]
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
#[derive(Debug)]
pub struct Unit {
    owner: Owner,
    health: Life,
    unit_type: UnitType,
}
#[derive(Debug)]
pub enum UnitType {
    Infantry,
    Mech,
    Reccon,
    Tank,
}

#[derive(Debug)]
pub struct MapCell {
    terrain: Terrain,
    building: Option<Building>,
    unit: Option<Unit>,
}

#[derive(Asset, TypePath)]
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

fn parse_v1(map: &Table) -> Result<Map, MapLoaderError> {
    let Some(width) = map["width"].as_integer() else {
        return Err(MapLoaderError::ParseError("Missing width or is not an integer".into()))
    };

    let Some(height) = map["height"].as_integer() else {
        return Err(MapLoaderError::ParseError("Missing height or is not an integer".into()))
    };
    let width = width as usize;
    let height = height as usize;

    let mut columns = Vec::new();
    columns.reserve(height as usize);

    let Some(terrain) = map["terrain"].as_array() else {
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
            row[idx] = MapCell {
                terrain: terrain_cell,
                building: None,
                unit: None,
            };
        }
        columns[idy] = row;
    }
    Ok(Map{cells: columns})
}

fn parse_map(content: &str) -> Result<Map, MapLoaderError> {
    let raw_file: Table = toml::from_str(content).map_err(|err| {
        bevy::log::error!("Error loading map {err}");
        MapLoaderError::ParseError("Invalid file format".into())
    })?;
    let version = raw_file["version"].as_integer();

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
