use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TDM {
    pub cardinality: usize,
    pub m: Vec<(usize, usize, usize)>,
}
// TODO getters