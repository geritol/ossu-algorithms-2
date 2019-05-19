use std::cmp::Ordering;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Edge {
    pub weight: i32,
    pub source: i32, // Source node's id
    pub target: i32, // Target node's id
}

impl Ord for Edge {
    fn cmp(&self, other: &Edge) -> Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Edge) -> Option<Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn odering_edge_smaller() {
        let negative_edge = Edge{weight: -1, target: 10, source: 1};
        let big_edge = Edge{weight: 10, target: 11, source: 2};
        assert_eq!(negative_edge > big_edge, false)
    }
}