use rand::seq::SliceRandom; // For random sampling
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn calculate_distance(node1: (f64, f64), node2: (f64, f64)) -> f64 {
    let dx = node1.0 - node2.0;
    let dy = node1.1 - node2.1;
    (dx * dx + dy * dy).sqrt()
}

fn read_nodes(roadNet-CA.txt: &str) -> io::Result<Vec<(f64, f64)>> {
    let path = Path::new(file_path);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    
    let mut nodes: Vec<(f64, f64)> = Vec::new();
    
    for line in reader.lines() {
        let line = line?;
        let coordinates: Vec<&str> = line.split_whitespace().collect();
        if coordinates.len() == 2 {
            let x = coordinates[0].parse::<f64>().unwrap();
            let y = coordinates[1].parse::<f64>().unwrap();
            nodes.push((x, y));
        }
    }
    Ok(nodes)
}