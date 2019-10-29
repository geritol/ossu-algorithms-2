use std::collections::HashMap;

pub struct UnionFind {
    pub nodes: HashMap<i32, Option<i32>>
}

impl UnionFind {
    pub fn new() -> UnionFind {
        UnionFind{nodes: HashMap::new()}
    }

    pub fn union(&mut self, node_1: i32, node_2: i32) {
        let node_1_leader = self.find_and_create_if_not_present(node_1);
        let node_2_leader = self.find_and_create_if_not_present(node_2);
        self.update_leader(node_2_leader, Some(node_1_leader));
    }

    pub fn is_in_different_cluster(&mut self, node_1: i32, node_2: i32 ) -> bool {
        let node_1_leader = self.find_and_create_if_not_present(node_1);
        let node_2_leader = self.find_and_create_if_not_present(node_2);
        node_1_leader != node_2_leader
    }

    fn find_and_create_if_not_present(&mut self, node: i32) -> i32 {
        match self.find_leader(node) {
            None => {
                self.update_leader(node, None);
                node
            },
            Some(node) => node
        }
    }

    fn update_leader(&mut self, node: i32, leader: Option<i32>) {
        self.nodes.insert(node, leader);
    }

    pub fn find_leader(&mut self, node: i32) -> Option<i32> {
        match self.nodes.get(&node) {
            None => return None, // Node not present 
            Some(leader) => {
                match leader {
                    None => return Some(node), // Node is the leader of itself
                    Some(leader) => {
                        let leader = self.find_leader(*leader);
                        self.nodes.insert(node, leader);
                        leader
                    }
                }   
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn union_of_two_non_present() {
        let mut union = UnionFind::new();
        union.union(1, 2);

        assert_eq!(union.find_leader(1), Some(1));
        assert_eq!(union.find_leader(2), Some(1))
    }

    #[test]
    fn find_not_present() {
        let mut union = UnionFind::new();
        union.nodes.insert(1, None);
        assert_eq!(union.find_leader(2), None)
    }

    #[test]
    fn find_self_leader() {
        let mut union = UnionFind::new();
        union.nodes.insert(1, None);
        assert_eq!(union.find_leader(1), Some(1))
    }

    #[test]
    fn find_nested_leader() {
        let mut union = UnionFind::new();
        union.nodes.insert(1, None);
        union.nodes.insert(2, Some(1));
        union.nodes.insert(3, Some(2));
        assert_eq!(union.find_leader(3), Some(1));
        assert_eq!(*union.nodes.get(&3).unwrap(), Some(1)) // It caches the result
    }
}
