//tests.rs

#[cfg(test)]

mod test_csv_to_txt {
    use super::csv_to_txt::{read_csv, remove_duplicates, write_txt, csv_to_txt};

    #[test]
    fn test_read_csv() {
        let csv_data = "Alice / Bob, Charlie\nDavid, Eve";
        let result = read_csv(&csv_data).unwrap();
        assert_eq!(result, vec![("Alice", "Bob"), ("Charlie", "David"), ("David", "Eve")]);
    }

    #[test]
    fn test_remove_duplicates() {
        let edges = vec![("Alice", "Bob"), ("Charlie", "David"), ("David", "Eve")];
        let result = remove_duplicates(&edges);
        assert_eq!(result, vec![("Alice", "Bob"), ("Charlie", "David"), ("David", "Eve")]);
    }


    #[test]
    fn test_write_txt() {
        let edges = vec![("Alice", "Bob"), ("Charlie", "David"), ("Eve", "David")];
        let output_path = "test_output.txt";
        assert!(write_txt(output_path, &edges).is_ok());

    }

    #[test]
    fn test_csv_to_txt_integration() {
        // Integration test for csv_to_txt
        let input_path = "test_input.csv";
        let output_path = "test_output.txt";

        // Create a test CSV file
        let csv_data = "Alice / Bob, Charlie\nDavid, Eve";
        std::fs::write(input_path, csv_data).expect("Failed to create test CSV file");

        // Run the conversion
        assert!(csv_to_txt(input_path, output_path).is_ok());

 
    }
}

#[cfg(test)]
mod test_data_loader {
    use super::data_loader::{get_hero_id, load_data};

    #[test]
    fn test_get_hero_id() {
        let mut id_map = HashMap::new();
        let mut next_id = 0;
        let result = get_hero_id("Alice", &mut id_map, &mut next_id);
        assert_eq!(result, HeroId("0".to_string()));
    }

    #[test]
    fn test_load_data() {
        // Add a test CSV file for loading
        let csv_data = "Alice / Bob, Charlie\nDavid, Eve";
        let file_path = "test_data_loader.csv";
        std::fs::write(file_path, csv_data).expect("Failed to create test CSV file");

        let result = load_data(file_path).unwrap();
        assert_eq!(result.0, vec![(HeroId("0".to_string()), HeroId("1".to_string())), (HeroId("2".to_string()), HeroId("3".to_string()))]);

        // Clean up: Delete the test CSV file
        std::fs::remove_file(file_path).expect("Failed to remove test CSV file");
    }
}


#[cfg(test)]
mod test_six_degrees {
    use super::six_degrees::six_degrees_of_separation;

    #[test]
    fn test_six_degrees_of_separation() {
        let mut network = HashMap::new();
        network.insert(HeroId("Alice".to_string()), HashSet::new());
        network.insert(HeroId("Bob".to_string()), HashSet::new());
        network.get_mut(&HeroId("Alice".to_string())).unwrap().insert(HeroId("Bob".to_string()));

        let start_node = HeroId("Alice".to_string());
        let result = six_degrees_of_separation(&network, &start_node);
        assert_eq!(result, HashMap::new());
    }
}
