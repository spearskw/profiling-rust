use std::io::BufRead;
use std::path::Path;
use itertools::Itertools;
use crate::models::{CostMatrix};

impl CostMatrix {
    pub fn from_file(path: &Path) -> CostMatrix {
        let file = std::fs::File::open(path).unwrap();
        let reader = std::io::BufReader::new(file);

        let cost_vec = reader.lines()
            .map(|line| parse_line(&line.unwrap()))
            .collect();

        CostMatrix {
            cost_vec
        }
    }

    pub fn find_cost(&self, origin_id: usize, destination_id: usize) -> isize {
        self.cost_vec[origin_id][destination_id]
    }
}

fn parse_line(line: &str) -> Vec<isize> {
    let chunks = line.chars().chunks(2);
    let costs = chunks.into_iter()
        .map(|mut chunk| chunk.join(""));

    costs.map(|chunk| chunk.parse::<isize>().unwrap()).collect()
}
