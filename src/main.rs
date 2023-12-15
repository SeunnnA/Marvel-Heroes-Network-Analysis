//main.rs

mod data_loader;
mod six_degrees;

use data_loader::HeroId;
use six_degrees::six_degrees_of_separation;
use std::collections::{HashMap, HashSet};
use std::env;
use rand::seq::SliceRandom;

// Function to build a network from a vector of edges
fn build_network(edges: &Vec<(HeroId, HeroId)>) -> HashMap<HeroId, HashSet<HeroId>> {
    let mut network: HashMap<HeroId, HashSet<HeroId>> = HashMap::new();

    for &(ref node1, ref node2) in edges {
        // Add edge from node1 to node2
        network
            .entry(node1.clone())
            .or_insert_with(HashSet::new)
            .insert(node2.clone());

        // Add edge from node2 to node1 (undirected network)
        network
            .entry(node2.clone())
            .or_insert_with(HashSet::new)
            .insert(node1.clone());
    }

    network
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <csv_file_path>", args[0]);
        std::process::exit(1);
    }

    let csv_file_path = &args[1];

    println!("Loading data from: {}", csv_file_path);

    // Load the cleaned data
    let (edges, _id_map) = data_loader::load_data(csv_file_path).expect("Failed to load data");

    //debugging to display the loaded edges.
    // println!("Loaded Edges: {:?}", edges.iter().take(10).collect::<Vec<_>>());

    // Choose a random starting node
    let start_node = choose_random_starting_node(&edges);

    //debugging to display the start node.
    // println!("Start Node: {}", start_node.0);

    // Build the network
    let network = build_network(&edges);

    //debugging to display the built network.
    // println!("Network: {:?}", network.iter().take(10).collect::<Vec<_>>());

    let degrees_map = six_degrees_of_separation(&network, &start_node);
    // Count degrees of separation for each hero
    let mut degree_counts: HashMap<usize, usize> = HashMap::new();
    for (_, degrees) in degrees_map.iter() {
        *degree_counts.entry(*degrees).or_insert(0) += 1;
    }

    //debugging to display the degree distribution.
    // println!("Degree Distribution:");
    // for (degrees, count) in &degree_counts {
    //     println!("Degrees {}: {}", degrees, count);
    // }

    // Find the hero with the highest degree
    let (hero_with_highest_degree, highest_degree) = degrees_map
        .iter()
        .max_by_key(|(_, degrees)| *degrees)
        .unwrap_or((&start_node, &0));

    println!(
        "Hero {:?} has the highest degree of separation: {}",
        hero_with_highest_degree, highest_degree
    );

    // Identify heroes with the highest degree
    let heroes_with_highest_degree: Vec<_> = degrees_map
        .iter()
        .filter(|(_, degrees)| *degrees == highest_degree)
        .map(|(hero, _)| hero)
        .collect();

    println!(
        "Heroes with the highest degree of separation ({}): {:?}",
        highest_degree, heroes_with_highest_degree
    );

    //debugging to display the degrees of separation.
    // for (hero, degrees) in degrees_map {
    //     println!("{:?} is {} degrees away from {:?}", hero, degrees, start_node);
    // }
}

fn choose_random_starting_node(edges: &Vec<(HeroId, HeroId)>) -> HeroId {
    // Extract unique hero names from the edges
    let unique_hero_names: Vec<&String> = edges
        .iter()
        .flat_map(|&(ref node1, ref node2)| vec![&node1.0, &node2.0])
        .collect();
    
    // Shuffle the names randomly
    let mut rng = rand::thread_rng();
    let mut shuffled_names = unique_hero_names.clone();
    shuffled_names.shuffle(&mut rng);

    // Choose a random name from the shuffled list
    let random_name = shuffled_names.choose(&mut rng).unwrap();

    HeroId(random_name.to_owned().to_string())
}