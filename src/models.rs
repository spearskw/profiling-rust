
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Node {
    pub id: usize,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Edge {
    pub origin_id: usize,
    pub destination_id: usize,
    pub cost: isize,
}

pub struct CostMatrix {
    pub cost_vec: Vec<Vec<isize>>
}

pub struct TspProblem {
    pub nodes: Vec<Node>,
    pub cost_matrix: CostMatrix
}

pub struct TourChange {
    pub index_swap: [usize; 2]
}
