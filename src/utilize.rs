use std::collections::{HashMap, VecDeque};

pub fn bfs(start_node: i32, adjacency_list: &HashMap<i32, Vec<i32>>) -> HashMap<i32, i32> {
    let mut distances = HashMap::new();
    let mut queue = VecDeque::new();
    distances.insert(start_node, 0);
    queue.push_back(start_node);
    while let Some(current_node) = queue.pop_front() {
        let current_distance = *distances.get(&current_node).unwrap();
        if let Some(neighbors) = adjacency_list.get(&current_node) {
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