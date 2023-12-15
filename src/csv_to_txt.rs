//csv_to_txt.rs

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::Path;

// Function to convert a CSV file to a cleaned TXT file
pub fn csv_to_txt(input_path: &str, output_path: &str) -> Result<(), io::Error> {
    let edges = read_csv(input_path)?;
    let cleaned_edges = remove_duplicates(&edges);
    write_txt(output_path, &cleaned_edges)?;
    Ok(())
}

// Function to read a CSV file and create a vector of edges
fn read_csv(input_path: &str) -> Result<Vec<(String, String)>, io::Error> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    let mut edges = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let nodes: Vec<&str> = line.split(',').map(|s| s.trim()).collect();

        if nodes.len() == 2 {
            // Extract name and alias from the node string
            let name_alias: Vec<&str> = nodes[0].split('/').map(|s| s.trim()).collect();
            let name = name_alias[0].to_owned();
            let alias = name_alias.get(1).map(|&s| s.to_owned()).unwrap_or_default();
            edges.push((name, alias));
        }
    }

    Ok(edges)
}

// Function to remove duplicate edges from a vector
fn remove_duplicates(edges: &[(String, String)]) -> Vec<(String, String)> {
    let mut unique_edges = HashMap::new();

    for &(ref node1, ref node2) in edges {
        unique_edges.insert((node1.clone(), node2.clone()), ());
    }

    unique_edges.keys().cloned().collect()
}

// Function to write a vector of edges to a TXT file
fn write_txt(output_path: &str, edges: &[(String, String)]) -> Result<(), io::Error> {
    let output_path = Path::new(output_path);
    let file = File::create(output_path)?;
    let mut writer = BufWriter::new(file);

    for (node1, node2) in edges {
        writeln!(writer, "{},{}", node1, node2)?;
    }

    Ok(())
}