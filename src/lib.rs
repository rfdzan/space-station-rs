// #![warn(missing_docs)]
//! # space-station-rs
//!
//! Exploring Rust's type system by creating a space station game.
//!
//! Inspired by [No Boilerplate's](https://www.youtube.com/@NoBoilerplate) video on youtube:
//! ['Building a space station in Rust'](https://www.youtube.com/watch?v=7GzQArrek7A&pp=ygUdbm8gYm9pbGVycGxhdGUgcnVzdCBzcGFjZXNoaXA%3D)
//!

use environment_resources::EnvResource;
use prelude::{MotherShip, WorldSize};
use rand::{self, Rng};
/// Structs, Enums, and methods for free-flying resources.
pub mod environment_resources;
/// Experiemental code to test ideas.
pub mod experimentals;
/// Structs, Enums, and methods for motherships.
pub mod mother_ship;
/// Imports all necessary modules from this library for convenience.
pub mod prelude;
/// Structs, Enums, and methods for spaceships.
pub mod space_ship;
/// Tests for space-station-rs library.
#[allow(unused_imports)]
#[cfg(test)]
pub mod tests;
/// Set the world parameters.
pub mod world;
/// Shared trait for generic information of a ship.
pub trait GetResourceLevels {
    /// Returns the amount of resources available in a resource object.
    fn get_resource_amount(&self, _rsc: ResourceKind) -> i32 {
        0
    }
}
/// Displays information of an object.
pub trait GenericInfo {
    /// Displays a ship's general information.
    fn display_info(&self) {}
    /// Displays a ship's current resources.
    fn display_resources(&self) {}
    /// Display a ship's current storage.
    fn display_storage(&self) {}
}
/// Implements a level cap on resources for ships.
pub trait LevelCap {
    /// General minimum level cap.
    fn adjust_min_level(&mut self) {}
    /// General maximum level cap.
    fn adjust_max_level(&mut self) {}
}
/// Shared trait for ships that can transfer resources, be it receiving or giving.
pub trait TransferResources {
    /// Spend resources currently available on a ship.
    /// ## Examples
    /// ```
    /// # use space_station::prelude::*;
    /// # let World = World::randomize(WorldSize::new(100));
    /// let mut ada = MotherShip::new("Ada", &World);
    /// ada.give_resources(ResourceKind::FoodWater(1), 100);
    /// ```
    fn give_resources(&mut self, _rsc: ResourceKind, _current_level: i32) -> bool {
        true
    }
    /// Receive resources to a ship.
    /// - Requires a resource/another ship that is capable of transferring resources.
    /// ## Examples
    /// ```
    /// # use space_station::prelude::*;
    /// # let World = World::randomize(WorldSize::new(100));
    /// let mut ada = MotherShip::new("Ada", &World);
    /// let mut zeus = SpaceShip::new("Zeus", & World);
    /// zeus.receive_resources(ResourceKind::FoodWater(20), &mut ada);
    /// ```
    fn receive_resources<T>(&mut self, _rsc: ResourceKind, _mtr_shp: &mut T)
    where
        T: TransferResources,
    {
    }
    /// Store environment resources inside the resource storage of a ship.
    fn receive_to_storage(&mut self, _rsc: ResourceKind) -> bool {
        false
    }
    /// Implementation WIP
    fn get_env_resources(&mut self, _env_resource: &mut EnvResource) -> GameWarning {
        GameWarning::Nominal
    }
    fn offload_storage(&mut self, _target: &mut MotherShip) {}
}
/// Shared trait for ships that can move.
pub trait Move {
    /// Move a ship to a location, taking into account its fuel levels and distance.
    fn to_location(&mut self, _to: Coordinates) -> bool {
        false
    }
    /// Teleports a ship to a mothership. Consuming no fuel in the process.
    fn teleport(&mut self, _mtr_ship: &MotherShip) {}
}
/// Struct for storage, a way for ships to store resources mined from the environment.
#[derive(Debug, Clone, Copy)]
pub struct Storage {
    consumable: FoodWater,
    oxygen: Oxygen,
    fuel: Fuel,
}
impl Storage {
    /// Creates a new Storage with customized starting values.
    pub fn new(amount: i32) -> Storage {
        Storage {
            consumable: FoodWater::new(amount),
            oxygen: Oxygen(amount),
            fuel: Fuel(amount),
        }
    }
}
impl GetResourceLevels for Storage {
    fn get_resource_amount(&self, rsc: ResourceKind) -> i32 {
        match rsc {
            ResourceKind::Fuel(_) => self.fuel.0,
            ResourceKind::Oxygen(_) => self.oxygen.0,
            ResourceKind::FoodWater(_) => self.consumable.0,
        }
    }
}
impl LevelCap for Storage {
    fn adjust_min_level(&mut self) {
        self.consumable = FoodWater(std::cmp::max(self.consumable.0, 0));
        self.oxygen = Oxygen(std::cmp::max(self.oxygen.0, 0));
        self.fuel = Fuel(std::cmp::max(self.fuel.0, 0));
    }
}
pub enum GameWarning {
    ShipStorageFull,
    OutOfBounds,
    Unreachable,
    ResourceExhausted,
    Nominal,
}
/// Command enums for user input.
pub enum Commands {
    /// Move command.
    MoveTo,
    /// Mine environmental resources.
    Mine,
    /// Recharge a ship by teleporting to a mothership.
    Recharge,
    /// Show a space ship's info.
    SpaceShipInfo,
    /// Show a mother ship's info.
    MotherShipInfo,
    /// Show resources within a certain distance of the ship.
    Ping,
    /// Offload storage to a mothership.
    Offload,
    /// Default enum if no commands are given. Should do nothing.
    Empty,
}
/// Spaceship docking enums.
#[derive(Debug)]
pub enum SpaceShipDockStatus {
    /// Spaceship is docked.
    Docked,
    /// Spaceship is undocked.
    Undocked,
}
/// Mothership recharge enums.
#[derive(Debug)]
pub enum MotherShipRechargeStatus {
    /// Mothership is recharging a spaceship.
    Charging,
    /// Charging port is vacant.
    Idle,
}
/// Mothership docking enums.
#[derive(Debug)]
pub enum MotherShipDockStatus {
    /// A spaceship is currently docked on the mothership.
    Populated,
    /// Mothership docking area is empty.
    Empty,
}
#[derive(Debug, Clone, Copy)]
pub struct Oxygen(i32);
impl Oxygen {
    pub fn new(amount: i32) -> Oxygen {
        Oxygen(amount)
    }
    pub fn randomize(max: i32) -> Oxygen {
        let mut rng = rand::thread_rng();
        Oxygen(rng.gen_range(5..=max))
    }
}
impl LevelCap for Oxygen {
    fn adjust_max_level(&mut self) {
        self.0 = std::cmp::min(self.0, 100);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct FoodWater(i32);
impl FoodWater {
    pub fn new(amount: i32) -> FoodWater {
        FoodWater(amount)
    }
    pub fn randomize(max: i32) -> FoodWater {
        let mut rng = rand::thread_rng();
        FoodWater(rng.gen_range(5..=max))
    }
}
impl LevelCap for FoodWater {
    fn adjust_max_level(&mut self) {
        self.0 = std::cmp::min(self.0, 100)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Fuel(i32);
impl Fuel {
    pub fn new(amount: i32) -> Fuel {
        Fuel(amount)
    }
    pub fn randomize(max: i32) -> Fuel {
        let mut rng = rand::thread_rng();
        Fuel(rng.gen_range(5..=max))
    }
}
impl LevelCap for Fuel {
    fn adjust_max_level(&mut self) {
        self.0 = std::cmp::min(self.0, 100);
    }
}
/// The main resources of the game.
#[derive(Debug, Clone, Copy)]
pub enum ResourceKind {
    /// Consumables.
    FoodWater(i32),
    /// Breathable air.
    Oxygen(i32),
    /// Rocket fuel.
    Fuel(i32),
}
impl ResourceKind {
    /// Generate a resource with randomized variant and amount.
    pub fn randomize(max: i32) -> ResourceKind {
        let mut rng = rand::thread_rng();
        let val = rng.gen_range(0..=2);
        let range = 5..=max;
        match val {
            0 => ResourceKind::FoodWater(rng.gen_range(range)),
            1 => ResourceKind::Oxygen(rng.gen_range(range)),
            2 => ResourceKind::Fuel(rng.gen_range(range)),
            _ => ResourceKind::Fuel(rng.gen_range(range)),
        }
    }
}
impl LevelCap for ResourceKind {
    fn adjust_max_level(&mut self) {
        match self {
            Self::FoodWater(val) => {
                *val = std::cmp::min(*val, 100);
            }
            Self::Oxygen(val) => {
                *val = std::cmp::min(*val, 100);
            }
            Self::Fuel(val) => {
                *val = std::cmp::min(*val, 100);
            }
        };
    }
    fn adjust_min_level(&mut self) {
        match self {
            Self::FoodWater(val) => {
                *val = std::cmp::max(*val, 0);
            }
            Self::Oxygen(val) => {
                *val = std::cmp::max(*val, 0);
            }
            Self::Fuel(val) => {
                *val = std::cmp::max(*val, 0);
            }
        }
    }
}
#[derive(Debug)]
/// Variants for quadrants, considering the coordinates of an object in the game.
pub enum Quadrants {
    /// (x, y)
    First,
    /// (-x, y)
    Second,
    /// (-x, -y)
    Third,
    /// (x, -y)
    Fourth,
}
/// Implementation WIP
pub struct Location();
/// Coordinates of an object in the game.
#[derive(Debug, Clone, Copy)]
pub struct Coordinates {
    x: i32,
    y: i32,
    world_size: WorldSize,
}
impl Coordinates {
    /// Creates a new coordinate.
    pub fn new(get_x: i32, get_y: i32, w_size: WorldSize) -> Self {
        Coordinates {
            x: get_x,
            y: get_y,
            world_size: w_size,
        }
    }
    /// Randomly generates a coordinate, within bounds of the playable game area (WIP)
    pub fn randomize(w_size: WorldSize) -> Coordinates {
        let mut rng = rand::thread_rng();
        let (min, max) = w_size.get_values();
        Coordinates {
            x: rng.gen_range(min..max),
            y: rng.gen_range(min..max),
            world_size: w_size,
        }
    }
    pub fn get_values(&self) -> (i32, i32) {
        (self.x, self.y)
    }
    fn max_bounds(&self) -> bool {
        let mut is_valid = true;
        let (min, max) = self.world_size.get_values();
        let values = [self.x, self.y];
        for item in values.into_iter().enumerate() {
            let (idx, val) = item;
            if val < min || val > max {
                is_valid = false;
                let axis = if idx == 0 { "x" } else { "y" };
                println!("{axis} value is out of bounds: {val}.")
            }
        }
        is_valid
    }
    fn get_quadrants(&self) -> Quadrants {
        if self.x.is_positive() && self.y.is_positive() {
            Quadrants::First
        } else if self.x.is_negative() && self.y.is_positive() {
            Quadrants::Second
        } else if self.x.is_negative() && self.y.is_negative() {
            Quadrants::Third
        } else {
            Quadrants::Fourth
        }
    }
    fn get_distance(&self, from: Coordinates) -> f64 {
        let side_a = from.x - self.x;
        let side_b = from.y - self.y;
        let dest = f64::from(side_a.pow(2) + side_b.pow(2));
        dest.sqrt().floor()
    }
}
