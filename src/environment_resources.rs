use std::cell::RefCell;

use crate::prelude::*;
#[derive(Debug, Clone, Copy)]
/// Struct for resources available in the environment.
pub struct EnvResource {
    kind: ResourceKind,
    coordinates: Coordinates,
    id: i32,
}
impl EnvResource {
    /// Spawn resources with randomized values.
    pub fn randomize(at_most: i32, id_num: i32, play_area: WorldSize) -> EnvResource {
        EnvResource {
            kind: ResourceKind::randomize(at_most),
            coordinates: Coordinates::randomize(play_area),
            id: id_num,
        }
    }
    /// Returns the resource's kind.
    pub fn get_kind(&self) -> ResourceKind {
        self.kind
    }
    /// Returns the resource's coordinates.
    pub fn get_coordinates(&self) -> Coordinates {
        self.coordinates
    }
    /// Returns the id of an environment resource.
    pub fn get_id(&self) -> i32 {
        self.id
    }
    /// Spawn a vector of resources for the game world.
    pub fn randomize_world_resources(
        amount: usize,
        at_most: i32,
        play_area: WorldSize,
    ) -> Vec<RefCell<EnvResource>> {
        let mut rsc_vec = Vec::with_capacity(amount);
        let convert_amount_to_i32 = i32::try_from(amount);
        let amount_as_i32 = match convert_amount_to_i32 {
            Ok(val) => val,
            Err(e) => {
                println!("Error converting world resource amount to i32\n\n{e}");
                0
            }
        };
        for num in 0..=amount_as_i32 {
            rsc_vec.push(RefCell::new(EnvResource::randomize(
                at_most, num, play_area,
            )))
        }
        rsc_vec
    }
}
impl TransferResources for EnvResource {
    fn give_resources(&mut self, _rsc: ResourceKind, _: i32) -> bool {
        match self.get_kind() {
            ResourceKind::FoodWater(val) => {
                if let ResourceKind::FoodWater(rate) = _rsc {
                    if val - rate != -1 {
                        self.kind = ResourceKind::FoodWater(val - rate);
                        true
                    } else {
                        false
                    }
                } else {
                    true
                }
            }
            ResourceKind::Oxygen(val) => {
                if let ResourceKind::Oxygen(rate) = _rsc {
                    if val - rate != -1 {
                        self.kind = ResourceKind::Oxygen(val - rate);
                        true
                    } else {
                        false
                    }
                } else {
                    true
                }
            }
            ResourceKind::Fuel(val) => {
                if let ResourceKind::Fuel(rate) = _rsc {
                    if val - rate != -1 {
                        self.kind = ResourceKind::Fuel(val - rate);
                        true
                    } else {
                        false
                    }
                } else {
                    true
                }
            }
        }
    }
}
