use std::fs::File;
use std::io::{BufRead, BufReader};

mod edge;
mod union_find;
use union_find::UnionFind;
use edge::Edge;

fn main() {
    let mut union_find = UnionFind::new();
    let (node_number,mut edges) = read_edges_from_file("clustering1.txt");
    edges.sort();

    let mut cluster_count = node_number;
    let mut maximum_spacing = 0;
    let target_cluster_count = 4;
    for edge in edges {
        if union_find.is_in_different_cluster(edge.source, edge.target) {
            maximum_spacing = edge.weight;
            if (cluster_count -1) < target_cluster_count {
                break;
            }
            union_find.union(edge.source, edge.target);
            cluster_count -=1;
        }
    }
    println!("Cluster count = {:?}, maximum spacing: {}",cluster_count, maximum_spacing);
}

fn read_edges_from_file(file_name: &str) -> (i32, Vec<Edge>){
    let file = File::open(file_name).expect("Unable to open");
    let mut edges = Vec::new();
    let mut node_number: i32 = 0;
    for (index, line) in BufReader::new(file).lines().enumerate() {
        if index == 0 {
            let unwrapped_line = line.unwrap();
            let split: Vec<&str> = unwrapped_line.split(' ').collect();
            node_number = split[0].parse::<i32>().unwrap();
        } else {
            let unwrapped_line = line.unwrap();
            let split: Vec<&str> = unwrapped_line.split(' ').collect();
            let source = split[0].parse::<i32>().unwrap();
            let target = split[1].parse::<i32>().unwrap();
            let weight = split[2].parse::<i32>().unwrap();
            edges.push(Edge{source, target, weight});
        }
    }
    (node_number, edges)
}