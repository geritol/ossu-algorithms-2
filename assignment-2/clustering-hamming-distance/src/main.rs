use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

mod node;
mod union_find;
use union_find::UnionFind;
use node::Node;

fn main() {
    let mut union_find = UnionFind::new();
    let (node_number, nodes_map, nodes_list) = read_nodes_from_file("clustering_big.txt");
    println!("File loaded");

    let mut cluster_count = node_number;
    for node_bits_i32 in nodes_list {
        let node = nodes_map.get(&node_bits_i32).unwrap();
        for edge in node.within_2.iter() {
            let edge_i32 = node::bits_to_i32(&edge);
            let is_node_present = match nodes_map.get(&edge_i32) {
                Some(x) => true,
                None => false
            };
            if is_node_present & (union_find.is_in_different_cluster(node_bits_i32, edge_i32)) {
                union_find.union(node_bits_i32, edge_i32);
                cluster_count -=1;
                println!("Cluster count: {}", cluster_count)
            }
        }
    }
    println!("Cluster count = {:?}", cluster_count);
}

fn read_nodes_from_file(file_name: &str) -> (i32, HashMap<i32, Node>, Vec<i32>){
    let file = File::open(file_name).expect("Unable to open");
    let mut nodes_map = HashMap::new();
    let mut nodes_list = Vec::new();
    let mut node_number: i32 = 0;
    for (index, line) in BufReader::new(file).lines().enumerate() {
        if index == 0 {
            let unwrapped_line = line.unwrap();
            let split: Vec<&str> = unwrapped_line.split(' ').collect();
            node_number = split[0].parse::<i32>().unwrap();
        } else {
            let unwrapped_line = line.unwrap();
            let node = Node::new(unwrapped_line);
            nodes_list.push(node.bits_i32);
            nodes_map.insert(node.bits_i32, node);
        }
    }
    (node_number, nodes_map, nodes_list)
}