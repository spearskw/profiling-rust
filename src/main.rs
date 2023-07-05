mod generate_problem;
mod models;
mod solver;

use std::path::Path;
use crate::generate_problem::create_random_problem;
use crate::solver::{calc_cost, find_best_tour};

fn main() {
    let path = Path::new("./matrix.txt");
    let problem = create_random_problem(100, path);
    let best_tour = find_best_tour(&problem, 100, path);
    let best_tour_cost = calc_cost(&best_tour, path);

    let best_tour_ids: Vec<u32> = best_tour.iter().map(|node| node.id).collect();
    println!("Best tour is: {:?}", best_tour_ids);
    println!("Best tour cost: {best_tour_cost}");
}
