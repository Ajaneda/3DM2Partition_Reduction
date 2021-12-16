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
        let mut carry = self.values[last_index] & second_summand.values[last_index];
        for (index, value) in self.values.iter().enumerate().rev().skip(1) {
            let current_xor_result = value ^ second_summand.values[index];
            result[index] = current_xor_result ^ carry;
            carry = value & second_summand.values[index] | carry & current_xor_result;
        }

        BinaryVector {
            values: result,
            p: self.p,
        }
    }
}

pub fn tdm_to_partition(tdm: &TDM, verbose: bool) -> Partition {
    let p = ((tdm.get_m().len() + 1) as f64).log2().ceil() as usize;
    let mut binary_rows = vec![];
    for (index, tuple) in tdm.get_m().iter().enumerate() {
        binary_rows.push(BinaryVector::new(tdm.get_cardinality(), p, *tuple));
        if verbose {
            println!(
                "s({})\t{} {}",
                index,
                binary_rows[index],
                binary_rows[index].to_decimal()
            );
        }
    }
    let c = binary_rows
        .iter()
        .fold(BinaryVector::empty(tdm.get_cardinality(), p), |acc, x| {
            &acc + &x
        });

    let b = BinaryVector::create_b(tdm.get_cardinality(), p);
    let b1 = c.to_decimal() * 2 - b.to_decimal();
    let b2 = c.to_decimal() + b.to_decimal();
    if verbose {
        println!("C\t{} {}", c, c.to_decimal());
        println!("B\t{} {}", b, b.to_decimal());
        println!("b1\t{}", &b1);
        println!("b2\t{}", &b2);
    }
    let mut values: Vec<usize> = binary_rows.iter().map(|x| x.to_decimal()).collect();
    values.push(b1);
    values.push(b2);
    Partition { values }
}
