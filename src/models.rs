use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Node {
    pub id: u32,
}

#[derive(Debug, Clone)]
pub struct Edge {
    pub origin_id: u32,
    pub destination_id: u32,
    pub cost: u32,
}

pub struct CostMatrix {
    pub cost_map: HashMap<(u32, u32), u32>
}

pub struct TspProblem {
    pub nodes: Vec<Node>,
    pub cost_matrix: CostMatrix
}
