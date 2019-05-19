use std::fs::File;
use std::io::{BufRead, BufReader};

mod edge;
use edge::Edge;

fn main() {
    let mut edges = read_edges_from_file("clustering1.small.txt");
    println!("{:?}", edges[0]);
    edges.sort();
    println!("{:?}", edges[0])
}

fn read_edges_from_file(file_name: &str) -> Vec<Edge>{
    let file = File::open(file_name).expect("Unable to open");
    let mut edges = Vec::new();
    for (index, line) in BufReader::new(file).lines().enumerate() {
        if index == 0 {
            // TODO: better first line handling
        } else {
            let unwrapped_line = line.unwrap();
            let split: Vec<&str> = unwrapped_line.split(' ').collect();
            let source = split[0].parse::<i32>().unwrap();
            let target = split[1].parse::<i32>().unwrap();
            let weight = split[2].parse::<i32>().unwrap();
            edges.push(Edge{source, target, weight});
        }
    }
    edges
}