use std::path::Path;
use crate::models::{find_cost, Node};

use rand::thread_rng;
use rand::seq::SliceRandom;

pub fn find_best_tour(nodes: &[Node], num_iterations: u32, path: &Path) -> Vec<Node> {
    let best_tour = (0..num_iterations)
        .map(|i| {
            println!("Starting iteration {i}");
            i
        })
        .map(|_| shuffle(nodes))
        .min_by_key(|tour| calc_cost(tour, path));

    best_tour.unwrap()
}

fn shuffle(nodes: &[Node]) -> Vec<Node> {
    let mut a = nodes.to_vec();
    a.shuffle(&mut thread_rng());
    a
}

pub fn calc_cost(tour: &Vec<Node>, path: &Path) -> u32 {
    let costs = (1..tour.len())
        .map(|i| find_cost(path, tour[i-1].id, tour[i].id));

    let final_edge_cost = find_cost(path, tour.last().unwrap().id, tour.first().unwrap().id);

    costs.sum::<u32>() + final_edge_cost
}