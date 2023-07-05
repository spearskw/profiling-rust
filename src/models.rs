
#[derive(Debug, Clone)]
pub struct Node {
    pub id: usize,
}

#[derive(Debug, Clone)]
pub struct Edge {
    pub origin_id: usize,
    pub destination_id: usize,
    pub cost: usize,
}

pub struct CostMatrix {
    pub cost_vec: Vec<Vec<usize>>
}

pub struct TspProblem {
    pub nodes: Vec<Node>,
    pub cost_matrix: CostMatrix
}
