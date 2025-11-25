use bevy::prelude::*;

use crate::{board::UnitType, interactive::Movement};


#[derive(Default)]
pub struct PlayerInfo {
    pub current_states: UnitsState
}


pub struct UnitsState {
    pub infantry: UnitState,
    pub mech: UnitState,
    pub recon: UnitState,
    pub tank: UnitState,
}

impl UnitsState {
    pub fn get(&self, unit_type: UnitType) -> &UnitState {
        match unit_type{
            UnitType::Infantry => &self.infantry,
            UnitType::Mech => &self.mech,
            UnitType::Recon => &self.recon,
            UnitType::Tank => &self.tank
            // _ => panic!("Unit type {:?} not supported", unit_type)
        }
    }
}

impl Default for UnitsState {
    fn default() -> Self {
        Self {
            infantry: UnitState::infantry(),
            mech: UnitState::mech(),
            recon: UnitState::recon(),
            tank: UnitState::tank()
        }
    }
}

pub struct UnitState {
    pub movement: Movement
}

impl UnitState {
    fn infantry() -> Self {
        Self {
            movement: Movement::infantry()
        }
    }
    fn mech() -> Self {
        Self {
            movement: Movement::mech()
        }
    }
    fn tank() -> Self {
        Self {
            movement: Movement::tank()
        }
    }
    fn recon() -> Self {
        Self {
            movement: Movement::recon()
        }
    }
}
