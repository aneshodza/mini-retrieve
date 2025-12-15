use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::types::DocId;

pub fn extract_queries(query_path: &str) -> HashMap<DocId, String> {

    let file = File::open(query_path).expect("Unable to open query file");
    let reader = BufReader::new(file);

    let mut queries: HashMap<DocId, String> = HashMap::new();
    let mut current_query_id = 0;
    let mut current_query = String::new();

    for line in reader.lines() {
        let parsed_line = line.unwrap_or(String::new());
        if parsed_line.starts_with(".I") {
            current_query_id = parsed_line
                .split_whitespace()
                .last()
                .unwrap_or("0")
                .parse::<DocId>()
                .unwrap_or(0);

            if !current_query.is_empty() {
                queries.insert(current_query_id, current_query.trim().to_string());
                current_query.clear();
            }
        } else if !parsed_line.starts_with(".W") {
            current_query.push_str(" ");
            current_query.push_str(&parsed_line);
        }
    }

    queries.insert(current_query_id, current_query.trim().to_string());
    queries
}
