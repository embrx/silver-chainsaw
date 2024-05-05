// analysis.rs

use std::collections::HashMap;
use std::collections::HashSet;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::utilize::bfs;

pub fn calculate_average_distance(sample: &[(i32, i32)], adjacency_list: &HashMap<i32, Vec<i32>>) -> f64 {
    let mut total_distance = 0.0;
    let mut total_pairs = 0;

    for &(node1, node2) in sample {
        let distances = bfs(node1, adjacency_list);
        let distances2 = bfs(node2, adjacency_list);

        total_distance += distances.values().sum::<i32>() as f64;
        total_distance += distances2.values().sum::<i32>() as f64;
        total_pairs += distances.len() - 1; // Exclude distance to itself
        total_pairs += distances2.len() - 1; // Exclude distance to itself
    }

    if total_pairs == 0 {
        return f64::INFINITY;
    }

    total_distance / total_pairs as f64
}

pub fn get_random_sample(adjacency_list: &HashMap<i32, Vec<i32>>, sample_size: usize) -> Vec<(i32, i32)> {
    let unique_edges: Vec<(i32, i32)> = adjacency_list.iter()
        .flat_map(|(&node1, neighbors)| neighbors.iter().map(move |&node2| (node1, node2)))
        .collect();
    let mut rng = thread_rng();
    let mut sample = unique_edges.clone();
    sample.shuffle(&mut rng);
    sample.truncate(sample_size);
    sample
}

pub fn calculate_unique_nodes_in_sample(sample: &[(i32, i32)]) -> usize {
    let mut unique_nodes_set = HashSet::new();

    for &(node1, node2) in sample {
        unique_nodes_set.insert(node1);
        unique_nodes_set.insert(node2);
    }

    unique_nodes_set.len()
}
