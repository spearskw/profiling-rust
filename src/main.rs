mod generate_problem;
mod models;
mod solver;
mod cost_matrix;
mod solver_test;

use std::path::Path;
use crate::generate_problem::create_random_problem;
use crate::solver::{calc_cost, find_best_tour};

fn main() {
    let path = Path::new("./matrix.txt");
    let problem = create_random_problem(1000, path);
    let best_tour = find_best_tour(&problem, 1000000);
    let best_tour_cost = calc_cost(&best_tour, &problem.cost_matrix);

    let best_tour_ids: Vec<usize> = best_tour.iter().map(|node| node.id).collect();
    println!("Best tour is: {:?}", best_tour_ids);
    println!("Best tour cost: {best_tour_cost}");
}
