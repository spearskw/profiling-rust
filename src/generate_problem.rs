use std::fs::{OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;
use rand::Rng;
use crate::models::{CostMatrix, Node, TspProblem};

pub fn create_random_problem(num_nodes: usize, path: &Path) -> TspProblem {
    write_file(num_nodes, path);
    let nodes = (0..num_nodes).map(|id| Node { id }).collect();
    let cost_matrix = CostMatrix::from_file(path);
    TspProblem {
        nodes,
        cost_matrix
    }
}

fn write_file(num_nodes: usize, path: &Path) {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)
        .unwrap();
    let mut writer = BufWriter::new(file);
    let mut rng = rand::thread_rng();
    for i in 0..num_nodes {
        for j in 0..num_nodes {
            if i == j {
                write!(writer, "00").unwrap();
            } else {
                write!(writer, "{}", rng.gen_range(10..100)).unwrap();
            }
        }
        writeln!(writer).unwrap();
    }
    writer.flush().unwrap();
}