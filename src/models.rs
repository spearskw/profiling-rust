use std::io::BufRead;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Node {
    pub id: u32,
}

pub fn find_cost(path: &Path, origin_id: u32, destination_id: u32) -> u32 {
    let file = std::fs::File::open(path).unwrap()   ;
    let reader = std::io::BufReader::new(file);
    let edge = reader.lines()
        .skip(1)
        .map(|l| parse_line(l.unwrap()))
        .find(|edge| edge.origin_id == origin_id && edge.destination_id == destination_id);

    edge.unwrap().cost
}


fn parse_line(line: String) -> Edge {
    let parts: Vec<&str> = line.split(',').collect();
    Edge {
        origin_id: parts[0].trim().parse::<u32>().unwrap(),
        destination_id: parts[1].trim().parse::<u32>().unwrap(),
        cost: parts[2].trim().parse::<u32>().unwrap(),
    }
}

#[derive(Debug, Clone)]
pub struct Edge {
    pub origin_id: u32,
    pub destination_id: u32,
    pub cost: u32,
}
