# Marvel-Heroes-Network-Analysis
explore the six degrees connnection between Marvel superheroes using graph theory and network analysis


Explore the connections between Marvel superheroes using graph theory and network analysis. This Rust-based project cleans, loads, and analyzes the dataset to uncover insights about the connections within the Marvel universe.


## Overview

The Marvel Heroes Network Analysis project explores the relationships between Marvel superheroes using graph theory and network analysis. This Rust-based project cleans, loads, and analyzes the dataset to uncover insights about the connections within the Marvel universe.

## Getting Started

To get started with the project, clone the repository:

```bash
git clone https://github.com/your-username/marvel-heroes-network-analysis.git
cd marvel-heroes-network-analysis
bash''' 
```

Then run the project:

```bash
cargo run hero_network.csv
```

## This is what is going on behind the scenes

## Data Cleaning
The raw dataset (`hero_network.csv`) undergoes cleaning to remove duplicates and format the data for analysis. The cleaned data is saved to `cleaned_data.txt`.

## Data Loading
The cleaned data is loaded into the program, assigning each hero a unique identifier (`HeroId`). This step also involves building a network representation of hero relationships.

## Network Building
The `build_network` function constructs a graph representation of hero relationships using the Petgraph library. It creates a network where heroes are nodes, and relationships are edges.

## Network Analysis
The `six_degrees_of_separation` function employs breadth-first search to calculate the degrees of separation between heroes, providing insights into the network structure.

## Results
The program outputs various results, including the loaded edges, the built network, degree distributions, and information about the hero with the highest degree.


