pub use land::*;
use rocket::serde::{Deserialize, Serialize};
use std::ops::{Index, IndexMut};

mod land;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(crate = "rocket::serde")]
pub enum MapType {
    Random,
}

impl Default for MapType {
    fn default() -> Self {
        Self::Random
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct MapConfig {
    pub size: usize,
    pub tp: MapType,
    pub player_count: i32,
    pub mountain_density: f64,
    pub city_density: f64,
}

impl Default for MapConfig {
    fn default() -> Self {
        Self {
            size: 20,
            tp: MapType::Random,
            player_count: 0,
            mountain_density: 0.13,
            city_density: 0.05,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Map {
    pub config: MapConfig,
    gm: Vec<Vec<Land>>,
}

impl Map {
    pub fn new(size: usize) -> Self {
        Self {
            config: MapConfig {
                size,
                ..Default::default()
            },
            gm: vec![vec![Land::default(); size + 1]; size + 1],
        }
    }
}

impl Index<usize> for Map {
    type Output = [Land];

    fn index(&self, index: usize) -> &Self::Output {
        &self.gm[index]
    }
}

impl IndexMut<usize> for Map {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.gm[index]
    }
}
