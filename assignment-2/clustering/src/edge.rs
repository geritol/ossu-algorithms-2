use std::cmp::Ordering;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Edge {
    pub weight: i32,
    pub target: i32, // Target node's id
}

// Reverse ordering
impl Ord for Edge {
    fn cmp(&self, other: &Edge) -> Ordering {
        other.weight.cmp(&self.weight)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Edge) -> Option<Ordering> {
        other.weight.partial_cmp(&self.weight)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn odering_edge_smaller() {
        let negative_edge = Edge{weight: -1, target: 10};
        let big_edge = Edge{weight: 10, target: 11};
        assert_eq!(negative_edge > big_edge, true)
    }
}