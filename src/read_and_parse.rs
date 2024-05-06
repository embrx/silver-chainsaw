use std::collections::HashMap; // methods
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

pub fn read_and_parse(filename: &str) -> Result<HashMap<i32, Vec<i32>>> {
    let file = File::open(filename)?; // this sets file to open
    let reader = BufReader::new(file); // reads text file
    let mut nodes = HashMap::new(); // assigns nodes variable
    let mut lines = reader.lines().skip(4); // skips four lines in file
    while let Some(Ok(line)) = lines.next() {
        let mut parts = line.split_whitespace(); // removes white space
        let node1: i32 = parts.next().unwrap().parse().unwrap(); // list of first nodes
        let node2: i32 = parts.next().unwrap().parse().unwrap(); // list of second nodes
        nodes.entry(node1).or_insert_with(Vec::new).push(node2);
        nodes.entry(node2).or_insert_with(Vec::new).push(node1);
    }
    Ok(nodes)
}