use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TDM {
  cardinality: usize,
  m: Vec<(usize, usize, usize)>,
}