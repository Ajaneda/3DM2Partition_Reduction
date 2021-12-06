use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TDM {
  cardinality: usize,
  m: Vec<(usize, usize, usize)>,
}