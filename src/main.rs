mod data_parsing;
mod analysis;
mod utilize;

use data_parsing::read_and_parse;
use analysis::{average_distance, random_sample, unique_nodes};
use std::time::Instant;

fn main() {
    let filename = "roadNet-CA.txt";

    println!("Starting the program...");

    // Reading the file and parsing data
    let start_time = Instant::now();
    println!("Reading the file and parsing data...");
    
    match read_file_and_parse_data(filename) {
        Ok(adjacency_list) => {
            println!("Data parsed successfully.");
            let elapsed = start_time.elapsed();
            println!("Time taken to parse data: {:.2?}", elapsed);

            let sample_size = 10;
            // Random sample of nodes
            println!("Getting a random sample of nodes (edges)...");
            let sample = get_random_sample(&adjacency_list, sample_size);
            println!("Sample obtained.");

            // Calculate the number of unique nodes in the sample
            let num_unique_nodes = calculate_unique_nodes_in_sample(&sample);

            // Calculating average distance
            println!("Calculating average distance...");
            let average_distance = calculate_average_distance(&sample, &adjacency_list);
            println!("Average distance calculated.");
            
            // Printing results
            println!("Sample size: {}", sample.len());
            println!("First 20 lines of the random sample:");
            let display_limit = sample.len().min(20); // Only show the first 20 lines if available
            let display_sample = &sample[..display_limit];
            println!("{:?}", display_sample);
            println!("Number of unique nodes in sample: {}", num_unique_nodes);
            println!("Average distance between nodes: {:.2}", average_distance);
        }
        Err(e) => {
            eprintln!("Failed to read the file: {}", e);
        }
    }

    println!("Program finished.");
}
