use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

pub fn read_file_and_parse_data(filename: &str) -> Result<HashMap<i32, Vec<i32>>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut adjacency_list = HashMap::new();

    // Skip the first 4 lines
    let mut lines = reader.lines().skip(4);
    
    while let Some(Ok(line)) = lines.next() {
        let mut parts = line.split_whitespace();
        let node1: i32 = parts.next().unwrap().parse().unwrap();
        let node2: i32 = parts.next().unwrap().parse().unwrap();

        // Add the connection to the adjacency list
        adjacency_list.entry(node1).or_insert_with(Vec::new).push(node2);
        adjacency_list.entry(node2).or_insert_with(Vec::new).push(node1);
    }
    
    Ok(adjacency_list)
}