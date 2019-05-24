struct Node {
    pub bits: [bool; 24]
}

fn get_distance_from_bits<const T: usize>(bits: [bool; T]) -> Vec<[bool; T]> {
    let mut result : Vec<[bool; T]> = Vec::new();
    for (i, value) in bits.iter().enumerate() {
        let mut mutation = [true; T];
        mutation.copy_from_slice(bits[..]);
        mutation[i] = !mutation[i];
        result.push(mutation)
    }
    result
}

impl Node {
    pub fn new(string: String) -> Node {
        let mut collector = [true; 24];
        for (i, character) in string.split(' ').into_iter().enumerate() {
            collector[i] = character == "1";
        }
        Node{bits: collector}
    }
    fn get_hamming_distance(&self, other: Node) -> u8 {
        let mut dist = 0;
        for (i, character) in self.bits.iter().enumerate() {
            if *character != other.bits[i] {
                dist += 1
            }
        }
        dist
    }

    fn generate_within_distance(&self, distance: u8) -> Vec<[bool; 24]> {
        get_distance_from_bits(self.bits)
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hamming_dist_0() {
        let node_1 = Node::new(String::from("0 1 1 0 0 1 1 0 0 1 0 1 1 1 1 1 1 0 1 0 1 1 0 1"));
        let node_2 = Node::new(String::from("0 1 1 0 0 1 1 0 0 1 0 1 1 1 1 1 1 0 1 0 1 1 0 1"));
        assert_eq!(node_1.get_hamming_distance(node_2), 0)
    }
     #[test]
    fn hamming_dist_3() {
        let node_1 = Node::new(String::from("0 1 1 0 0 1 1 0 0 1 0 1 1 1 1 1 1 0 1 0 1 1 0 1"));
        let node_2 = Node::new(String::from("0 1 0 0 0 1 0 0 0 1 0 1 1 1 1 1 1 0 1 0 0 1 0 1"));
        assert_eq!(node_1.get_hamming_distance(node_2), 3)
    }

    #[test]
    fn generate_within_distance_1() {
        let node = Node::new(String::from("0 0 0 0"));
        let within_1 = node.generate_within_distance(1);
        let result : Vec<[bool; 24]> = Vec::new();
        assert_eq!(within_1, result)
    }
}