//six_degrees.rs

use std::collections::{HashMap, HashSet};
use crate::data_loader::HeroId;

// Function to find degrees of separation for each hero in the network
pub fn six_degrees_of_separation(
    network: &HashMap<HeroId, HashSet<HeroId>>,
    start_node: &HeroId,
) -> HashMap<HeroId, usize> {
    let mut visited = HashMap::new();
    let mut queue = std::collections::VecDeque::new();

    queue.push_back((start_node.clone(), 0));
    visited.insert(start_node.clone(), 0);

    while let Some((node, depth)) = queue.pop_front() {
        if let Some(neighbors) = network.get(&node) {
            for neighbor in neighbors {
                if !visited.contains_key(neighbor) {
                    visited.insert(neighbor.clone(), depth + 1);
                    queue.push_back((neighbor.clone(), depth + 1));
                }
            }
        }
        // debugging to display the visited nodes.
        //println!("Node: {:?}, Depth: {}", node, depth);
    }

    visited
}
