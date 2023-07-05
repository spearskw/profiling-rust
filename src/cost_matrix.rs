use std::collections::HashMap;
use std::io::BufRead;
use std::path::Path;
use crate::models::{CostMatrix, Edge};

impl CostMatrix {
    pub fn from_file(path: &Path) -> CostMatrix {
        let file = std::fs::File::open(path).unwrap();
        let reader = std::io::BufReader::new(file);

        let mut cost_map = HashMap::new();

        let edges = reader.lines()
            .skip(1)
            .map(|line| parse_line(line.unwrap()));

        for edge in edges {
            let pair = (edge.origin_id, edge.destination_id);
            cost_map.insert(pair, edge.cost);
        }

        CostMatrix {
            cost_map
        }
    }

    pub fn find_cost(&self, origin_id: u32, destination_id: u32) -> u32 {
        let pair = (origin_id, destination_id);
        let cost = self.cost_map.get(&pair);
        *cost.unwrap()
    }
}

fn parse_line(line: String) -> Edge {
    let parts: Vec<&str> = line.split(',').collect();
    Edge {
        origin_id: parts[0].trim().parse::<u32>().unwrap(),
        destination_id: parts[1].trim().parse::<u32>().unwrap(),
        cost: parts[2].trim().parse::<u32>().unwrap(),
    }
}
