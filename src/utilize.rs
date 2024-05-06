use std::collections::{HashMap, VecDeque}; // methods

pub fn bfs(start_node: i32, nodes: &HashMap<i32, Vec<i32>>) -> HashMap<i32, i32> { // bfs calculation
    let mut distances = HashMap::new(); // sets distance variable
    let mut queue = VecDeque::new(); // sets queue variable which
    distances.insert(start_node, 0);
    queue.push_back(start_node);
    while let Some(current_node) = queue.pop_front() { // iterates through nodes
        let current_distance = *distances.get(&current_node).unwrap();
        if let Some(neighbors) = nodes.get(&current_node) {
            for &neighbor in neighbors {
                if !distances.contains_key(&neighbor) {
                    distances.insert(neighbor, current_distance + 1);
                    queue.push_back(neighbor);
                }
            }
        }
    }
    distances
}