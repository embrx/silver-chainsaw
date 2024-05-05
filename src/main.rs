mod read_and_parse;
mod analysis;
mod utilize;

use read_and_parse::read_and_parse;
use analysis::{average_distance, random_sample, unique_nodes, clustering_coefficient};
use std::time::Instant;

fn main() {
    let filename = "roadNet-CA.txt"; // filename
    println!("Starting the program...");
    let start_time = Instant::now(); // reading and parsing data
    println!("Reading and parsing the file...");
    match read_and_parse(filename) {
        Ok(nodes) => {
            println!("Data parsed successfully.");
            let elapsed = start_time.elapsed();
            println!("Time taken to parse data: {:.2?}", elapsed);

            let sample_size = 1000; // adjust sample size here
            println!("Getting a random sample...");
            let sample = random_sample(&nodes, sample_size);
            println!("Sample obtained.");
            let elapsed = start_time.elapsed();
            println!("Time taken to get sample: {:.2?}", elapsed);

            let unique_nodes_count = unique_nodes(&sample); // calculate unique nodes
            println!("Calculating average distance..."); // average distance
            let avg_distance = average_distance(&sample, &nodes);
            println!("Average distance calculated.");
            let elapsed = start_time.elapsed();
            println!("Time taken to calculate average distances: {:.2?}", elapsed);

            println!("Calculating clustering coefficient...");
            let cl_coefficient = clustering_coefficient(&nodes);
            println!("Clustering coefficient calculated.");
            let elapsed = start_time.elapsed();
            println!("Time taken to calculate clustering coefficient: {:.2?}", elapsed);

            println!("Sample size: {}", sample.len()); // printing results
            println!("First 20 lines of the random sample:");
            let display_limit = sample.len().min(20); // only shows first 20
            let display_sample = &sample[..display_limit];
            println!("{:?}", display_sample);
            println!("Number of unique nodes in sample: {}", unique_nodes_count);
            println!("Average distance between nodes: {:.2}", avg_distance);
            println!("Average clustering coefficient: {:.4}", cl_coefficient);
        }
        Err(e) => {
            eprintln!("Failed to read the file: {}", e);
        }
    }
    println!("Program finished.");
}