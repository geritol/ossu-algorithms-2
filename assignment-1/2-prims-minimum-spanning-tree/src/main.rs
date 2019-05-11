mod edge;
mod node;
mod graph;
use graph::Graph;

fn main() {

    // Build Graph
    let mut graph = Graph::new();
    graph.add_edge(1,2,2);
    graph.add_edge(1,4,-1);
    graph.add_edge(2,4, 4);
    graph.add_edge(3,1,3);
    graph.add_edge(4,3,5);

    // Call function
    println!("Hello, world! \n the response is {}", get_mst_length(&mut graph));
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
