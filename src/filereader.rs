use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use rand::prelude::SliceRandom;

pub fn read_nodes(file_path: &str) -> HashSet<(f64, f64)> {
    // Create a set to store unique nodes
    let mut nodes: HashSet<(f64, f64)> = HashSet::new();

    // Open the file for reading
    let file = File::open(file_path).expect("Failed to open file");
    let reader = io::BufReader::new(file);

    // Read each line of the file
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let coords: Vec<&str> = line.split_whitespace().collect();

        // Parse and add coordinates to the set
        if coords.len() == 2 {
            let x = coords[0].parse::<f64>().expect("Failed to parse coordinate");
            let y = coords[1].parse::<f64>().expect("Failed to parse coordinate");

            // Add the node to the set (automatically handles duplicates)
            nodes.insert((x, y));
        }
    }

    nodes
}