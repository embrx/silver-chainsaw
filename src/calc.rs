pub fn calculate_distance(node1: &(f64, f64), node2: &(f64, f64)) -> f64 {
    let (x1, y1) = node1;
    let (x2, y2) = node2;

    let dx = x1 - x2;
    let dy = y1 - y2;

    (dx * dx + dy * dy).sqrt()
}

pub fn calculate_average_distance(
    sample: &[&(f64, f64)],
    nodes_vec: &[(f64, f64)],
) -> f64 {
    let mut distances: Vec<f64> = Vec::new();

    for &sample_node in sample {
        for &node in nodes_vec {
            let distance = calculate_distance(sample_node, &node);
            distances.push(distance);
        }
    }

    let sum: f64 = distances.iter().sum();
    sum / distances.len() as f64
}