use std::collections::HashMap; // methods
use std::collections::HashSet;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::utilize::bfs;

pub fn average_distance(sample: &[(i32, i32)], nodes: &HashMap<i32, Vec<i32>>) -> f64 {
    let mut total_distance = 0.0;
    let mut total_pairs = 0;
    for &(node1, node2) in sample { // distance formula
        let distances = bfs(node1, nodes);
        let distances2 = bfs(node2, nodes);
        total_distance += distances.values().sum::<i32>() as f64;
        total_distance += distances2.values().sum::<i32>() as f64;
        total_pairs += distances.len() - 1;
        total_pairs += distances2.len() - 1;
    }
    if total_pairs == 0 {
        return f64::INFINITY;
    }
    total_distance / total_pairs as f64
}

pub fn random_sample(nodes: &HashMap<i32, Vec<i32>>, sample_size: usize) -> Vec<(i32, i32)> {
    let unique_edges: Vec<(i32, i32)> = nodes.iter()
        .flat_map(|(&node1, neighbors)| neighbors.iter().map(move |&node2| (node1, node2)))
        .collect();
    let mut rng = thread_rng();
    let mut sample = unique_edges.clone();
    sample.shuffle(&mut rng);
    sample.truncate(sample_size);
    sample
}

pub fn unique_nodes(sample: &[(i32, i32)]) -> usize {
    let mut unique_nodes_set = HashSet::new();
    for &(node1, node2) in sample {
        unique_nodes_set.insert(node1);
        unique_nodes_set.insert(node2);
    }
    unique_nodes_set.len()
}

pub fn clustering_coefficient(nodes: &HashMap<i32, Vec<i32>>) -> f64 {
    let mut total_coefficient = 0.0;
    let mut valid_nodes = 0;
    for (&node, neighbors) in nodes.iter() {
        let degree = neighbors.len();
        if degree < 2 {
            continue;
        }
        let mut triangles = 0;
        for (i, &neighbor1) in neighbors.iter().enumerate() {
            for &neighbor2 in &neighbors[i + 1..] {
                if nodes.get(&neighbor1).map_or(false, |n1_neighbors| n1_neighbors.contains(&neighbor2)) {
                    triangles += 1;
                }
            }
        }
        let coefficient = (2 * triangles) as f64 / (degree * (degree - 1)) as f64;
        total_coefficient += coefficient;
        valid_nodes += 1;
    }
    if valid_nodes == 0 {
        return 0.0;
    }
    total_coefficient / valid_nodes as f64
}