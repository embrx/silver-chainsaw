mod calc;
mod filereader;

use calc::calculate_average_distance;
use filereader::read_nodes;
use rand::thread_rng;
use rand::prelude::SliceRandom;

fn main() {
    // Specify the file path
    let file_path = "roadNet-CA.txt";

    // Read nodes from file
    let nodes_set = read_nodes("roadNet-CA.txt");
    let nodes_vec: Vec<(f64, f64)> = nodes_set.into_iter().collect();

    // Define the sample size (choose a reasonable sample size)
    let sample_size = (nodes_vec.len() as f64 * 0.1) as usize; // 10% of nodes, for example

    // Randomly sample the nodes
    let mut rng = thread_rng();
    let sample: Vec<&(f64, f64)> = nodes_vec
        .choose_multiple(&mut rng, sample_size)
        .cloned()
        .collect();

    // Calculate average distance
    let average_distance = calculate_average_distance(&sample, &nodes_vec);

    // Print the average distance
    println!("Average distance: {:.4}", average_distance);
}