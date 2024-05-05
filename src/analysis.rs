use std::collections::{HashMap};
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::utilize::bfs;

pub fn calculate_average_distance(sample: &[i32], adjacency_list: &HashMap<i32, Vec<i32>>) -> f64 {
    let mut total_distance = 0.0;
    let mut total_pairs = 0;

    for &node in sample {
        let distances = bfs(node, adjacency_list);
        total_distance += distances.values().sum::<i32>() as f64;
        total_pairs += distances.len() - 1; // Exclude distance to itself
    }

    if total_pairs == 0 {
        return f64::INFINITY;
    }

    total_distance / total_pairs as f64
}

pub fn get_random_sample(adjacency_list: &HashMap<i32, Vec<i32>>, sample_size: usize) -> Vec<i32> {
    let unique_nodes: Vec<i32> = adjacency_list.keys().cloned().collect();
    let mut rng = thread_rng();
    let mut sample = unique_nodes.clone();
    sample.shuffle(&mut rng);
    sample.truncate(sample_size);
    sample
}
