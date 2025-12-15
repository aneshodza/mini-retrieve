pub fn extract_queries(query_path: &str) -> Vec<String> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let file = File::open(query_path).expect("Unable to open query file");
    let reader = BufReader::new(file);
    let mut queries = Vec::new();
    let mut current_query = String::new();

    for line in reader.lines() {
        let parsed_line = line.unwrap_or(String::new());
        if parsed_line.starts_with(".I") {
            if !current_query.is_empty() {
                queries.push(current_query.clone());
                current_query.clear();
            }
        } else if !parsed_line.starts_with(".W") {
            current_query.push_str(&parsed_line);
        }
    }
    queries
}
