use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TDM {
    cardinality: usize,
    m: Vec<(usize, usize, usize)>,
}

impl TDM {
    pub fn get_cardinality(&self) -> usize {
        self.cardinality
    }

    pub fn get_m(&self) -> &Vec<(usize, usize, usize)> {
        &self.m
    }
}
// TODO getters