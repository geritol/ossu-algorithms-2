use std::fs::File;
use std::io::{BufRead, BufReader};

mod edge;
mod node;
mod graph;
use graph::Graph;

fn main() {

    // Build Graph
    let mut graph = build_graph_from_file("edges.small.txt");

    // Call function
    println!("The response is {}", get_mst_length(&mut graph));
}

fn get_mst_length(graph: &mut Graph) -> i32{
    let mut visited = vec![1];
    let mut total_weight = 0;
    let mut go_on = true;
    while go_on {
        let result = graph.get_lowest_unvisited_edge(&visited);
        if result.0 == None {
            go_on = false;
        } else {
            total_weight += result.1.unwrap().weight;
            visited.push(result.1.unwrap().target);
        }
    }
    total_weight
}

fn build_graph_from_file(file_name: &str) -> Graph{
    let file = File::open(file_name).expect("Unable to open");
    let mut graph = Graph::new();
    for (index, line) in BufReader::new(file).lines().enumerate() {
        if index == 0 {
            // TODO: better first line handling
        } else {
            let unwrapped_line = line.unwrap();
            let split: Vec<&str> = unwrapped_line.split(' ').collect();
            let from = split[0].parse::<i32>().unwrap();
            let to = split[1].parse::<i32>().unwrap();
            let weight = split[2].parse::<i32>().unwrap();
            graph.add_edge(from,to,weight);
        }
    }
    graph
}
