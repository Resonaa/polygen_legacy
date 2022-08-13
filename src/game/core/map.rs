use super::land::*;
use rocket::serde::{Deserialize, Serialize};
use std::ops::{Index, IndexMut};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(crate = "rocket::serde")]
pub enum MapMode {
    Hexagon,
    Quadrilateral,
}

impl Default for MapMode {
    fn default() -> Self {
        Self::Hexagon
    }
}

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
    pub mode: MapMode,
    pub tp: MapType,
    pub player_count: i32,
}

impl Default for MapConfig {
    fn default() -> Self {
        Self {
            size: 20,
            mode: MapMode::default(),
            tp: MapType::default(),
            player_count: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Map {
    pub config: MapConfig,
    gm: Vec<Vec<Land>>,
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
