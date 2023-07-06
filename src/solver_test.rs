#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::models::{CostMatrix, Edge, Node, TourChange};
    use crate::solver::{calc_cost, calc_cost_delta, find_neighboring_nodes, find_new_edges, find_old_edges};

    #[test]
    fn calc_cost_delta_works() {
        let mut tour = vec![Node { id: 0 }, Node { id: 1 }, Node { id: 2 }];
        let cost_vec = vec![
            vec![0, 1, 1],
            vec![1, 0, 1],
            vec![1, 1, 0],
        ];
        let cost_matrix = CostMatrix {
            cost_vec
        };
        let tour_change = TourChange {index_swap: [0, 2]};
        let delta_cost = calc_cost_delta(&mut tour, &tour_change, &cost_matrix);
        assert_eq!(delta_cost, 0);
    }

    #[test]
    fn find_neighboring_nodes_works() {
        let tour = vec![Node { id: 0 }, Node { id: 1 }, Node { id: 2 }];

        assert_eq!(find_neighboring_nodes(&tour, 0), [&tour[2], &tour[1]]);
        assert_eq!(find_neighboring_nodes(&tour, 1), [&tour[0], &tour[2]]);
        assert_eq!(find_neighboring_nodes(&tour, 2), [&tour[1], &tour[0]]);
    }

    #[test]
    fn find_old_edges_works() {
        let tour = vec![Node { id: 0 }, Node { id: 1 }, Node { id: 2 }];
        let cost_vec = vec![
            vec![0, 1, 1],
            vec![1, 0, 1],
            vec![1, 1, 0],
        ];
        let cost_matrix = CostMatrix {
            cost_vec
        };
        let tour_change = TourChange {index_swap: [0, 2]};
        let edges = find_old_edges(&tour, &tour_change, &cost_matrix);

        let mut expected_edges = HashSet::new();
        expected_edges.insert(Edge {origin_id: 2, destination_id: 0, cost: 1});
        expected_edges.insert(Edge {origin_id: 0, destination_id: 1, cost: 1});
        expected_edges.insert(Edge {origin_id: 1, destination_id: 2, cost: 1});
        assert_eq!(edges, expected_edges);
    }

    #[test]
    fn find_new_edges_works() {
        let original_tour = vec![Node { id: 0 }, Node { id: 1 }, Node { id: 2 }];
        let mut tour = original_tour.clone();
        let cost_vec = vec![
            vec![0, 1, 1],
            vec![1, 0, 1],
            vec![1, 1, 0],
        ];
        let cost_matrix = CostMatrix {
            cost_vec
        };
        let tour_change = TourChange {index_swap: [0, 2]};
        let edges = find_new_edges(&mut tour, &tour_change, &cost_matrix);

        let mut expected_edges = HashSet::new();
        expected_edges.insert(Edge {origin_id: 0, destination_id: 2, cost: 1});
        expected_edges.insert(Edge {origin_id: 2, destination_id: 1, cost: 1});
        expected_edges.insert(Edge {origin_id: 1, destination_id: 0, cost: 1});
        assert_eq!(edges, expected_edges);
        assert_eq!(tour, original_tour);
    }
}