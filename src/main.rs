// main.rs

mod data_parsing;
mod analysis;
mod utilize;

use data_parsing::read_file_and_parse_data;
use analysis::{calculate_average_distance, get_random_sample, calculate_unique_nodes_in_sample};
use std::time::Instant;

fn main() {
    let filename = "roadNet-CA.txt";

    println!("Starting the program...");

    // Step 1: Read the file and parse data
    let start_time = Instant::now();
    println!("Reading the file and parsing data...");
    
    match read_file_and_parse_data(filename) {
        Ok(adjacency_list) => {
            println!("Data parsed successfully.");
            let elapsed = start_time.elapsed();
            println!("Time taken to parse data: {:.2?}", elapsed);

            let sample_size = 10;
            // Step 2: Get a random sample of nodes (pairs of nodes)
            println!("Getting a random sample of nodes (edges)...");
            let sample = get_random_sample(&adjacency_list, sample_size);
            println!("Sample obtained.");

            // Calculate the number of unique nodes in the sample
            let num_unique_nodes = calculate_unique_nodes_in_sample(&sample);

            // Step 3: Calculate average distance
            println!("Calculating average distance...");
            let average_distance = calculate_average_distance(&sample, &adjacency_list);
            println!("Average distance calculated.");
            
            // Step 4: Print results
            println!("Sample size: {}", sample.len());
            println!("Number of unique nodes in sample: {}", num_unique_nodes);
            println!("Average distance between nodes: {:.2}", average_distance);
        }
        Err(e) => {
            eprintln!("Failed to read the file: {}", e);
        }
    }

    println!("Program finished.");
}
