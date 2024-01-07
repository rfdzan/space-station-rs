use crate::mother_ship::MotherShip;
use crate::{
    FoodWater, Fuel, GenericInfo, LevelCap, MotherShipDockStatus, MotherShipRechargeStatus, Oxygen,
    SpaceShipDockStatus, SpaceShipRecharge,
};
use rand::{self, prelude::*};
use std::thread::sleep;
use std::time::Duration;
#[derive(Debug)]
pub struct SpaceShip<'a> {
    name: &'a str,
    consumables: FoodWater,
    oxygen: Oxygen,
    fuel: Fuel,
    dock_status: SpaceShipDockStatus,
}
impl<'a> SpaceShip<'a> {
    fn docked(&mut self, mtr_shp: &mut MotherShip) {
        mtr_shp.dock = MotherShipDockStatus::Populated;
        mtr_shp.recharge = MotherShipRechargeStatus::Charging;
        self.dock_status = SpaceShipDockStatus::Docked;
    }
    fn undocked(&mut self, mtr_shp: &mut MotherShip) {
        mtr_shp.dock = MotherShipDockStatus::Empty;
        mtr_shp.recharge = MotherShipRechargeStatus::Idle;
        self.dock_status = SpaceShipDockStatus::Undocked;
    }
    fn recharge_backend(&mut self, mtr_shp: &mut MotherShip) {
        let FoodWater::Level(initial_consumable_level) = self.consumables;
        let Oxygen::Level(initial_oxygen_level) = self.oxygen;
        let Fuel::Level(initial_fuel_level) = self.fuel;
        let a = [
            initial_fuel_level,
            initial_oxygen_level,
            initial_consumable_level,
        ];
        let min = a.iter().min().unwrap_or(&0);
        mtr_shp.display_info();
        for _ in *min..100 {
            self.recharge_consumables(1);
            self.recharge_oxygen(1);
            self.recharge_fuel(1);
            sleep(Duration::from_millis(200));
            self.display_info();
        }
    }
    pub fn new(n: &'a str) -> SpaceShip<'a> {
        let mut rng = rand::thread_rng();
        let mut s = SpaceShip {
            name: n,
            consumables: FoodWater::Level(rng.gen_range(50..100)),
            oxygen: Oxygen::Level(rng.gen_range(50..100)),
            fuel: Fuel::Level(rng.gen_range(50..100)),
            dock_status: SpaceShipDockStatus::Undocked,
        };
        s.consumables.adjust_level();
        s.oxygen.adjust_level();
        s.fuel.adjust_level();
        s
    }
    pub fn recharge(&mut self, mtr_shp: &mut MotherShip) {
        self.docked(mtr_shp);
        self.recharge_backend(mtr_shp);
        self.undocked(mtr_shp);
    }
}
impl<'a> SpaceShipRecharge for SpaceShip<'a> {
    fn recharge_consumables(&mut self, rate: i32) {
        let FoodWater::Level(initial_consumable_level) = self.consumables;
        self.consumables = FoodWater::Level(initial_consumable_level + rate);
        self.consumables.adjust_level();
    }
    fn recharge_oxygen(&mut self, rate: i32) {
        let Oxygen::Level(initial_oxygen_level) = self.oxygen;
        self.oxygen = Oxygen::Level(initial_oxygen_level + rate);
        self.oxygen.adjust_level()
    }
    fn recharge_fuel(&mut self, rate: i32) {
        let Fuel::Level(initial_fuel_level) = self.fuel;
        self.fuel = Fuel::Level(initial_fuel_level + rate);
        self.fuel.adjust_level();
    }
}
impl<'a> GenericInfo for SpaceShip<'a> {
    fn display_info(&self) {
        let n = self.name;
        let FoodWater::Level(c) = self.consumables;
        let Oxygen::Level(o) = self.oxygen;
        let Fuel::Level(f) = self.fuel;
        println!("--Ship Status--\nName: {n}\nFood & Water: {c}\nOxygen: {o}\nFuel: {f}");
    }
}
