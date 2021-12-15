use super::partition::Partition;
use super::tdm::TDM;
use std::fmt;
use std::ops;

#[derive(Debug)]
struct BinaryVector {
    values: Vec<u8>,
    p: usize,
}

impl BinaryVector {
    fn new(n: usize, p: usize, (x, y, z): (usize, usize, usize)) -> Self {
        let mut values = vec![0; 3 * n * p];
        let groups = [x, y, z];
        for (index, item) in groups.iter().enumerate() {
            values[((item * p) - 1) + p * n * index] = 1;
        }
        Self { values, p }
    }

    fn empty(n: usize, p: usize) -> Self {
        let values = vec![0; 3 * n * p];
        Self { values, p }
    }

    fn create_b(n: usize, p: usize) -> Self {
        let mut values = vec![0; 3 * n * p];
        for index in 0..values.len() {
            if index % p == p - 1 {
                values[index] = 1;
            }
        }
        Self { values, p }
    }

    fn to_decimal(&self) -> usize {
        self.values.iter().fold(0, |acc, x| acc * 2 + *x as usize)
    }
}

impl fmt::Display for BinaryVector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let group_size = self.values.len() / 3;
        for (index, value) in self.values.iter().enumerate() {
            if index != 0 && index % group_size == 0 {
                write!(f, "   ")?;
            }
            write!(f, "{}", value)?;
            if index != 0 && index % self.p == self.p - 1 {
                write!(f, " ")?;
            }
        }
        Ok(())
    }
}

impl ops::Add<&BinaryVector> for &BinaryVector {
    type Output = BinaryVector;
    fn add(self, second_summand: &BinaryVector) -> BinaryVector {
        assert_eq!(self.values.len(), second_summand.values.len());
        assert_eq!(self.p, second_summand.p);

        let mut result = vec![0; self.values.len()];
        let last_index = result.len() - 1;
        result[last_index] = self.values[last_index] ^ second_summand.values[last_index];
        for (index, value) in self.values.iter().enumerate().rev().skip(1) {
            result[index] = value ^ second_summand.values[index]
                | self.values[index + 1] & second_summand.values[index + 1];
        }

        BinaryVector {
            values: result,
            p: self.p,
        }
    }
}

pub fn tdm_to_partition(tdm: &TDM) -> Partition {
    let p = ((tdm.m.len() + 1) as f64).log2().ceil() as usize;
    let mut binary_rows = vec![];
    for tuple in &tdm.m {
        binary_rows.push(BinaryVector::new(tdm.cardinality, p, *tuple));
        println!(
            "{} {}",
            binary_rows[binary_rows.len() - 1],
            binary_rows[binary_rows.len() - 1].to_decimal()
        );
    }
    let c = binary_rows
        .iter()
        .fold(BinaryVector::empty(tdm.cardinality, p), |acc, x| &acc + &x);

    let b = BinaryVector::create_b(tdm.cardinality, p);
    println!("{} {}", c, c.to_decimal());
    println!("{} {}", b, b.to_decimal());
    println!("b1: {}", c.to_decimal() * 2 - b.to_decimal());
    println!("b2: {}", c.to_decimal() + b.to_decimal());
    Partition {
        values: vec![1, 2, 3, 4, 5, 6],
    }
}
