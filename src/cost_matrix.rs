use std::io::BufRead;
use std::path::Path;
use crate::models::{CostMatrix, Edge};

impl CostMatrix {
    pub fn from_file(path: &Path) -> CostMatrix {
        let file = std::fs::File::open(path).unwrap();
        let reader = std::io::BufReader::new(file);

        let edges = reader.lines()
            .skip(1)
            .map(|line| parse_line(line.unwrap()));

        let mut cost_vec = Vec::new();
        println!("Growing vector");
        for edge in edges {
            if edge.origin_id >= cost_vec.len() {
                cost_vec.insert(edge.origin_id, Vec::new());
            }
            cost_vec[edge.origin_id].insert(edge.destination_id, edge.cost);
        }
        println!("Vector grown");

        CostMatrix {
            cost_vec
        }
    }

    pub fn find_cost(&self, origin_id: usize, destination_id: usize) -> isize {
        self.cost_vec[origin_id][destination_id]
    }
}

fn parse_line(line: String) -> Edge {
    let parts: Vec<&str> = line.split(',').collect();
    Edge {
        origin_id: parts[0].trim().parse::<usize>().unwrap(),
        destination_id: parts[1].trim().parse::<usize>().unwrap(),
        cost: parts[2].trim().parse::<isize>().unwrap(),
    }
}
