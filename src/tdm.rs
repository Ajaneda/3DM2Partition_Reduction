use super::utility;
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

    pub fn to_dot(&self) -> String {
        let mut result = String::from("digraph G {\nrankdir = LR;\n");
        let set_names = ["w", "x", "y"];
        for (index, set_name) in set_names.iter().enumerate() {
            result.push_str(&format!(
                "{}\n",
                TDM::subgraph(set_name, self.cardinality, index)
            ));
        }
        let colors = utility::get_colors(self.m.len());
        for (index, tuple) in self.m.iter().enumerate() {
            result.push_str(&format!(
                "{}{} -> {}{} -> {}{} [color=\"{}\"];\n",
                set_names[0], tuple.0, set_names[1], tuple.1, set_names[2], tuple.2, colors[index]
            ));
        }
        result.push_str("}");
        result
    }

    fn subgraph(current_set: &str, cardinality: usize, index: usize) -> String {
        let mut subgraph_created: String = format!("subgraph cluster_{} {{\n", index);
        subgraph_created.push_str("node [style=filled];\n");
        for i in 1..=cardinality {
            subgraph_created.push_str(&format!("{}{}", current_set, i));
            if i == cardinality {
                subgraph_created += ";\n";
            } else {
                subgraph_created += " ";
            }
        }
        subgraph_created.push_str(&format!(
            "label = \"{}\";\n}}\n",
            current_set.to_uppercase()
        ));
        subgraph_created
    }
}
