//data_loader.rs

use std::collections::HashMap;
use std::io::{self};
use std::fs::File;
use csv::ReaderBuilder;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct HeroId(pub String);

// Function to load data from a CSV file and create a vector of edges
pub fn load_data(file_path: &str) -> Result<(Vec<(HeroId, HeroId)>, HashMap<String, HeroId>), io::Error> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().has_headers(false).from_reader(file);

    let mut edges = Vec::new();
    let mut id_map = HashMap::new();
    let mut next_id = 0;

    for result in rdr.records() {
        let record = result?;
        if record.len() >= 2 {
            // Get or create HeroId for hero1 and hero2
            let hero1_id = get_hero_id(&record[0], &mut id_map, &mut next_id);
            let hero2_id = get_hero_id(&record[1], &mut id_map, &mut next_id);
            edges.push((hero1_id, hero2_id));
        }
    }

    Ok((edges, id_map))
}

// Function to get HeroId for a given name, creating a new one if necessary
fn get_hero_id(name: &str, id_map: &mut HashMap<String, HeroId>, next_id: &mut usize) -> HeroId {
    id_map.entry(name.to_string()).or_insert_with(|| {
        let id = *next_id;
        *next_id += 1;
        HeroId(id.to_string())
    }).clone()
}