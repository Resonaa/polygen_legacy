use super::land::*;
use rocket::serde::{Deserialize, Serialize};
use std::ops::{Index, IndexMut};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Map {
    pub size: usize,
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
