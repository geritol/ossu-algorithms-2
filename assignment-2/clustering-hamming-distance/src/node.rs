use std::collections::HashSet;

pub struct Node {
    pub bits: Vec<bool>,
    pub bits_i32: i32,
    pub within_2: HashSet<Vec<bool>>
}

pub fn bits_to_i32(bits: &Vec<bool>) -> i32 {
    let mut result = 0;
    for (i, bit) in bits.iter().enumerate(){
        if *bit {
            result += 2_i32.pow(i as u32)
        }
    }
    result
}

fn generate_within_distance_2(bits: &Vec<bool>) -> HashSet<Vec<bool>> {
    let mut result = HashSet::new();
    let distance_1_result = generate_within_distance_1(&bits);
    for bits in distance_1_result {
        result.insert(bits);
    }
    let distance_1_result = generate_within_distance_1(&bits);
    for bits in distance_1_result.iter(){
        let sub_result = generate_within_distance_1(&bits);
        for sub_result_bits in sub_result {
            result.insert(sub_result_bits);
        }
    }
    result
}

fn generate_within_distance_1(bits: &Vec<bool>) -> Vec<Vec<bool>> {
    let mut result = Vec::new();
    for (i, value) in bits.iter().enumerate() {
        let mut mutation = bits.clone();
        mutation[i] = !mutation[i];
        result.push(mutation)
    }
    result
}

impl Node {
    pub fn new(string: String) -> Node {
        let mut collector = Vec::new();
        for character in string.split(' ').into_iter() {
            collector.push(character == "1");
        }
        let within_2 = generate_within_distance_2(&collector);
        let bits_i32 = bits_to_i32(&collector);
        Node{bits: collector, bits_i32, within_2}
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

    fn generate_within_distance_2(&self) -> HashSet<Vec<bool>> {
        generate_within_distance_2(&self.bits)
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

    // #[test]
    // fn generate_within_distance_1() {
    //     let node = Node::new(String::from("0 0 0 0"));
    //     let within_1 = node.generate_within_distance(1);
    //     let expected = vec![vec![true, false, false, false], vec![false, true, false, false], vec![false, false, true, false], vec![false, false, false, true]];
    //     assert_eq!(within_1, expected)
    // }

    #[test]
    fn generate_within_distance_2() {
        let node = Node::new(String::from("0 0 0 0"));
        let within_2 = node.generate_within_distance_2();
        assert!(within_2.contains(&vec![true, false, false, false]));
        assert!(within_2.contains(&vec![false, true, false, false]));
        assert!(within_2.contains(&vec![false, false, true, false]));
        assert!(within_2.contains(&vec![false, false, false, true]));
        assert!(within_2.contains(&vec![true, true, false, false]));
        assert!(within_2.contains(&vec![true, false, true, false]));
        assert!(within_2.contains(&vec![true, false, false, true]));
        assert!(within_2.contains(&vec![false, true, true, false]));
        assert!(within_2.contains(&vec![false, true, false, true]));
        assert!(within_2.contains(&vec![false, false, true, true]));
    }
}