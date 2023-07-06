use std::collections::HashSet;
use itertools::Itertools;
use rand::rngs::ThreadRng;
use crate::models::{CostMatrix, Edge, Node, TourChange, TspProblem};

use rand::{Rng, thread_rng};

pub fn find_best_tour(problem: &TspProblem, num_iterations: usize) -> Vec<Node> {
    let mut current_tour = problem.nodes.clone();
    let mut current_cost = calc_cost(&current_tour, &problem.cost_matrix);
    let mut random = thread_rng();

    for i in 0..num_iterations {
        let tour_change = generate_tour_change(&current_tour, &mut random);
        let cost_delta = calc_cost_delta(&mut current_tour, &tour_change, &problem.cost_matrix);
        let new_cost = current_cost + cost_delta;
        if new_cost < current_cost {
            println!("Found new best cost {new_cost}");
            current_cost = new_cost;
            current_tour.swap(tour_change.index_swap[0], tour_change.index_swap[1]);
        }
    }

    current_tour
}

fn generate_tour_change(tour: &[Node], random: &mut ThreadRng) -> TourChange {
    let index1 = random.gen_range(0..tour.len());
    let mut index2 = random.gen_range(0..tour.len());
    while index1 == index2 {
        index2 = random.gen_range(0..tour.len());
    }

    TourChange {
        index_swap: [index1, index2]
    }
}

pub fn calc_cost_delta(tour: &mut [Node], tour_change: &TourChange, cost_matrix: &CostMatrix) -> isize {
    let old_edges = find_old_edges(tour, tour_change, cost_matrix);
    let new_edges = find_new_edges(tour, tour_change, cost_matrix);

    let old_cost: isize = old_edges.iter().map(|edge| edge.cost).sum();
    let new_cost: isize = new_edges.iter().map(|edge| edge.cost).sum();

    new_cost - old_cost
}

pub fn find_old_edges(tour: &[Node], tour_change: &TourChange, cost_matrix: &CostMatrix) -> HashSet<Edge> {
    let mut edges = HashSet::new();

    let neighbors = find_neighboring_nodes(tour, tour_change.index_swap[0]);

    let origin_id = neighbors[0].id;
    let destination_id = tour[tour_change.index_swap[0]].id;
    edges.insert(Edge {
        origin_id,
        destination_id,
        cost: cost_matrix.find_cost(origin_id, destination_id)
    });

    let origin_id = tour[tour_change.index_swap[0]].id;
    let destination_id =  neighbors[1].id;
    edges.insert(Edge {
        origin_id,
        destination_id,
        cost: cost_matrix.find_cost(origin_id, destination_id)
    });


    let neighbors = find_neighboring_nodes(tour, tour_change.index_swap[1]);

    let origin_id = neighbors[0].id;
    let destination_id = tour[tour_change.index_swap[1]].id;
    edges.insert(Edge {
        origin_id,
        destination_id,
        cost: cost_matrix.find_cost(origin_id, destination_id)
    });

    let origin_id = tour[tour_change.index_swap[1]].id;
    let destination_id =  neighbors[1].id;
    edges.insert(Edge {
        origin_id,
        destination_id,
        cost: cost_matrix.find_cost(origin_id, destination_id)
    });

    edges
}

pub fn find_new_edges(tour: &mut [Node], tour_change: &TourChange, cost_matrix: &CostMatrix) -> HashSet<Edge> {
    tour.swap(tour_change.index_swap[0], tour_change.index_swap[1]);
    let edges = find_old_edges(tour, tour_change, cost_matrix);
    tour.swap(tour_change.index_swap[0], tour_change.index_swap[1]);
    edges
}

pub fn find_neighboring_nodes(tour: &[Node], index: usize) -> [&Node; 2] {
    if index == 0 {
        return [tour.last().unwrap(), &tour[1]];
    }
    if index == tour.len() - 1 {
        return [&tour[index - 1], &tour[0]];
    }
    [&tour[index - 1], &tour[index + 1]]
}

pub fn calc_cost(tour: &Vec<Node>, cost_matrix: &CostMatrix) -> isize {
    let costs = (1..tour.len())
        .map(|i| cost_matrix.find_cost(tour[i - 1].id, tour[i].id));

    let final_edge_cost = cost_matrix.find_cost(tour.last().unwrap().id, tour.first().unwrap().id);

    costs.sum::<isize>() + final_edge_cost
}