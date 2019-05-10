use std::collections::BinaryHeap;

use crate::edge::Edge;

#[derive(Debug)]
pub struct Node {
    pub edges: BinaryHeap<Edge>
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_smallest_edge() {
        let mut node = Node{edges: BinaryHeap::new()};
        let negative_edge = Edge{weight: -1, target: 10};
        let big_edge = Edge{weight: 10, target: 11};
        node.edges.push(big_edge);
        node.edges.push(negative_edge);
        let smallest_edge = node.edges.pop();
        assert_eq!(smallest_edge.unwrap().weight, -1);
        assert_eq!(node.edges.len(), 1);
    }
}