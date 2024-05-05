use roads::{read_and_parse, analysis::{average_distance, random_sample, unique_nodes}};
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;

fn test_read_and_parse() {
    let filename = "test_roadNet.txt";
    let mut file = File::create(filename).unwrap();
    writeln!(file, "1 2").unwrap();
    writeln!(file, "2 3").unwrap();
    writeln!(file, "3 4").unwrap();

    let nodes = read_and_parse(filename);
    assert!(nodes.is_ok());
    let nodes = nodes.unwrap();

    let mut expected: HashSet<(usize, usize)> = HashSet::new();
    expected.insert((1, 2));
    expected.insert((2, 3));
    expected.insert((3, 4));

    assert_eq!(nodes, expected);

    std::fs::remove_file(filename).unwrap();
}

fn test_random_sample() {
    let nodes = vec![(1, 2), (2, 3), (3, 4)];
    let sample = random_sample(&nodes, 2);

    assert_eq!(sample.len(), 2);

    for edge in &sample {
        assert!(nodes.contains(edge));
    }
}

fn test_unique_nodes() {
    let sample = vec![(1, 2), (2, 3), (3, 4), (1, 2)];

    let unique_count = unique_nodes(&sample);

    assert_eq!(unique_count, 4);
}

fn test_average_distance() {
    let sample = vec![(1, 2), (2, 3), (3, 4)];

    let nodes: HashSet<(usize, usize)> = sample.iter().cloned().collect();

    let avg_dist = average_distance(&sample, &nodes);

    assert_eq!(avg_dist, (1 + 1 + 1) as f64 / 3.0);
}