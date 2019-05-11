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
    }

    pub fn get_lowest_cost_edge(&self, explored_nodes: &Vec<i32>) -> Option<Edge> {
        let mut lowest_cost_edge: Option<Edge> = None;
        let mut lowest_cost_edge_start: Option<&i32> = None;
        for node_index in explored_nodes.iter() {
            let lowest_edge = self.get_lowest_unvisited_edge(node_index, explored_nodes);
            if lowest_edge != None {
                if lowest_cost_edge == None {
                    lowest_cost_edge = lowest_edge;
                    lowest_cost_edge_start = Some(node_index);
                } else if lowest_edge > lowest_cost_edge {
                    lowest_cost_edge = lowest_edge;
                    lowest_cost_edge_start = Some(node_index);
                }
            }
        }
        // self.nodes.get_mut(lowest_cost_edge_start.unwrap()).unwrap().edges.pop();
        lowest_cost_edge
    }

    fn get_lowest_unvisited_edge(&self, node_index: &i32, explored_nodes: &Vec<i32>) -> Option<Edge>{
        let lowest_edge = self.nodes.get(node_index).unwrap().edges.peek();
        match lowest_edge {
            None => None,
            Some(edge) => {
                if explored_nodes.contains(&edge.target) {
                    // self.nodes.get_mut(node_index).unwrap().edges.pop();
                    self.get_lowest_unvisited_edge(node_index, explored_nodes)
                } else {
                    Some(edge.clone())
                }
            }
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
        let visited = vec![1];
        assert_eq!(graph.get_lowest_cost_edge(&visited).unwrap().weight, -1)
    }
}