use crate::models::{CostMatrix, Node, TspProblem};

use rand::thread_rng;
use rand::seq::SliceRandom;

pub fn find_best_tour(problem: &TspProblem, num_iterations: u32) -> Vec<Node> {
    let best_tour = (0..num_iterations)
        .map(|i| {
            println!("Starting iteration {i}");
            i
        })
        .map(|_| shuffle(&problem.nodes))
        .min_by_key(|tour| calc_cost(tour, &problem.cost_matrix));

    best_tour.unwrap()
}

fn shuffle(nodes: &[Node]) -> Vec<Node> {
    let mut a = nodes.to_vec();
    a.shuffle(&mut thread_rng());
    a
}

pub fn calc_cost(tour: &Vec<Node>, cost_matrix: &CostMatrix) -> u32 {
    let costs = (1..tour.len())
        .map(|i| cost_matrix.find_cost(tour[i-1].id, tour[i].id));

    let final_edge_cost = cost_matrix.find_cost( tour.last().unwrap().id, tour.first().unwrap().id);

    costs.sum::<u32>() + final_edge_cost
}