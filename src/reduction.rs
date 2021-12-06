use super::tdm::TDM;
use super::partition::Partition;

struct BinaryVector {
  values: Vec<u8>
}

impl BinaryVector {
  fn new(n: usize, p: usize, (x, y, z): (usize, usize, usize)) -> Self {
    let mut values = vec![0; 3 * n * p];
    let groups = [x, y, z];
    for (j, item) in groups.iter().enumerate() {
        values[item * p - 1 + n * j] = 1; // n == q ?????
    }
    Self { values }
  }
}

pub fn tdm_to_partition(tdm: &TDM) -> Partition {
  // let p = ((k + 1) as f64).log2().ceil() as usize;

  Partition { values: vec!(1, 2, 3, 4, 5, 6) } // TODO Borrar esto luego uwu
}