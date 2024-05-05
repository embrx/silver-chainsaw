mod data_parsing;
mod analysis;
mod utilize;

use data_parsing::read_and_parse;
use analysis::{average_distance, random_sample, unique_nodes};
use std::time::Instant;

fn main() {
    let filename = "roadNet-CA.txt";

    println!("Starting the program...");

    let start_time = Instant::now(); // reading and parsing data
    println!("Reading the file and parsing data...");
    
    match read_file_and_parse_data(filename) {
        Ok(adjacency_list) => {
            println!("Data parsed successfully.");
            let elapsed = start_time.elapsed();
            println!("Time taken to parse data: {:.2?}", elapsed);

            let sample_size = 10; // adjust sample size here
            println!("Getting a random sample of nodes (edges)...");
            let sample = random_sample(&adjacency_list, sample_size);
            println!("Sample obtained.");

            let num_unique_nodes = unique_nodes(&sample); // calculate unique nodes

            println!("Calculating average distance..."); // average distance
            let average_distance = average_distance(&sample, &adjacency_list);
            println!("Average distance calculated.");
            
            println!("Sample size: {}", sample.len()); // printing results
            println!("First 20 lines of the random sample:");
            let display_limit = sample.len().min(20); // only shows first 20
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
