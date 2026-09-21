use bevy::{platform::collections::HashMap, prelude::*};

use crate::{board::{Board, Direction, Terrain}, interactive::{Life, Owner}};

#[derive(Component)]
pub struct UnitDefinition{
    movement: MovementDefinition
}

pub struct MovementDefinition{
    points: u32,
    costs: HashMap<Terrain, u32>
}

impl MovementDefinition {
    pub fn cost(&self, terrain: &Terrain) -> Option<u32> {
        self.costs.get(terrain).copied()
    }
}


impl UnitDefinition {
    pub fn infantry() -> Self {
        UnitDefinition { movement: MovementDefinition { points: 30, costs: HashMap::from_iter([
            (Terrain::Plane, 10),
            (Terrain::Forest, 15),
            (Terrain::Road, 10),
            (Terrain::Mountain, 20),
        ]) } }
    }
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Unit {
    pub owner: Owner,
    pub health: Life,
    pub unit_type: UnitType,
}

impl UnitDefinition {
    pub fn get_movements(&self, pos: UVec2, board: &Board) -> Vec<PossibleMovement> {
        let mut movements: HashMap<UVec2, PossibleMovement> = HashMap::default();
        let mut pending_check = vec![PossibleMovement {
            cost: 0,
            layer: 0,
            position: pos,
        }];
        let total_movement = self.movement.points;
        while let Some(to_check) = pending_check.pop() {
            let is_new_or_better = match movements.get(&to_check.position) {
                Some(existing) => existing.cost > to_check.cost,
                None => true,
            };
            if is_new_or_better {
                for dir in Direction::ADJACENT {
                    bevy::log::info!("Direction: {:?}", dir);
                    let Some(new_pos) = dir.move_point(&to_check.position) else {
                        continue;
                    };
                    let Some(terrain) = board.get(&new_pos) else {
                        continue;
                    };
                    let Some(move_cost) = self.movement.cost(terrain) else {
                        continue;
                    };
                    let new_cost = to_check.cost + move_cost;
                    bevy::log::info!("New Cost: {}", new_cost);
                    if new_cost < total_movement {
                        pending_check.push(PossibleMovement {
                            position: new_pos,
                            layer: to_check.layer + 1,
                            cost: new_cost,
                        });
                    }
                }
                movements.insert(to_check.position, to_check);
            }
        }
        bevy::log::info!("We have possible movements! {}", movements.len());
        movements
            .into_iter()
            .map(|(_, movement)| movement)
            .collect()
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
    pub cost: u32,
}

impl UnitType {
    pub fn to_definition(&self) -> UnitDefinition{
        match self{
            Self::Infantry => UnitDefinition::infantry(),
            unit_type => todo!("Unimplemented unit type: {:?}", unit_type)
        }
    }
}

/* impl UnitType {
    pub fn get_movement(&self) -> u32 {
        match self {
            Self::Infantry => 30,
            Self::Mech => 25,
            Self::Reccon => 50,
            Self::Tank => 45,
        }
    }
} */
