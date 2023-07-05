use std::fs::{OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;
use rand::Rng;
use crate::models::{Node};

pub fn create_random_problem(num_nodes: u32, path: &Path) -> Vec<Node> {
    write_file(num_nodes, path);
    (0..num_nodes).map(|id| Node { id }).collect()
}

fn write_file(num_nodes: u32, path: &Path) {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)
        .unwrap();
    let mut writer = BufWriter::new(file);
    let mut rng = rand::thread_rng();
    writeln!(writer, "origin, destination, cost").unwrap();
    for i in 0..num_nodes {
        for j in 0..num_nodes {
            if i == j {
                writeln!(writer, "{i}, {j}, 0").unwrap();
            } else {
                writeln!(writer, "{i}, {j}, {}", rng.gen_range(10..100)).unwrap();
            }
        }
    }
    writer.flush().unwrap();
}