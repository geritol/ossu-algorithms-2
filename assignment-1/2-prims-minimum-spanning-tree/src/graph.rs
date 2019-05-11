use std::collections::HashMap;
use std::collections::BinaryHeap;

use crate::edge::Edge;
use crate::node::Node;

#[derive(Debug)]
pub struct Graph {
    pub nodes: HashMap<i32, Node>
}

impl Graph {
    pub fn new() -> Graph {
        Graph{ nodes: HashMap::new()}
    }

    pub fn add_edge(&mut self, from: i32, to: i32, weight: i32) {
        let edge = Edge{weight, target: to};
        match self.nodes.get_mut(&from) {
            Some(node) => node.edges.push(edge),
            None => self.add_node(from, edge)
        };
        let edge_2 = Edge{weight, target: from};
        match self.nodes.get_mut(&to) {
            Some(node) => node.edges.push(edge_2),
            None => self.add_node(to, edge_2)
        };
    }

    pub fn get_lowest_unvisited_edge(&mut self, explored_nodes: &Vec<i32>) -> (Option<i32>, Option<Edge>) {
        let result = &self.get_lowest_cost_edge(explored_nodes);
        if result.0 == None {
            return (None, None)
        }
        self.nodes.get_mut(&result.0.unwrap()).unwrap().edges.pop();
        if explored_nodes.contains(&result.1.unwrap().target) {
            return self.get_lowest_unvisited_edge(explored_nodes)
        }
        *result
    }

    fn get_lowest_cost_edge(&self, explored_nodes: &Vec<i32>) -> (Option<i32>, Option<Edge>) {
        let mut lowest_cost_edge: Option<Edge> = None;
        let mut lowest_cost_edge_start: Option<i32> = None;
        for node_index in explored_nodes.iter() {
            let lowest_edge = self.get_copy_of_lowest_edge(node_index);
            if lowest_edge == None {
                continue;
            }
            if lowest_cost_edge == None || lowest_edge > lowest_cost_edge {
                lowest_cost_edge = lowest_edge;
                lowest_cost_edge_start = Some(*node_index);
            }
        }
        (lowest_cost_edge_start ,lowest_cost_edge)
    }

    fn get_copy_of_lowest_edge(&self, node_index: &i32) -> Option<Edge>{
        let lowest_edge = &self.nodes[node_index].edges.peek();
        match lowest_edge {
            None => None,
            Some(edge) => Some(**edge)
        }
        
    }

    fn add_node(&mut self, from: i32, edge: Edge){
        let mut edges = BinaryHeap::new();
        edges.push(edge);
        self.nodes.insert(from, Node{edges});
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_new_edge() {
        let mut graph = Graph::new();
        graph.add_edge(1,2,2);
        graph.add_edge(1,3,3);
        assert_eq!(graph.nodes.get_mut(&1).unwrap().edges.pop().unwrap().target, 2)
    }

    #[test]
    fn lowest_cost_edge() {
        let mut graph = Graph::new();
        graph.add_edge(1,2,2);
        graph.add_edge(1,4,-1);
        graph.add_edge(2,4, 4);
        graph.add_edge(3,1,3);
        graph.add_edge(4,3,5);
        let mut visited = vec![1];

        let first_result = graph.get_lowest_unvisited_edge(&visited);
        assert_eq!(first_result.1.unwrap().weight, -1);
        visited.push(first_result.1.unwrap().target);

        let second_result = graph.get_lowest_unvisited_edge(&visited);
        assert_eq!(second_result.1.unwrap().weight, 2);
        visited.push(second_result.1.unwrap().target);

        let third_result = graph.get_lowest_unvisited_edge(&visited);
        assert_eq!(third_result.1.unwrap().weight, 3);
        visited.push(third_result.1.unwrap().target);

        let last_result = graph.get_lowest_unvisited_edge(&visited);
        assert_eq!(last_result.0, None);
        assert_eq!(last_result.1, None);
    }
}