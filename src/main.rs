use std::time::Instant;

use mini_retrieve::{
    commander::commander, querying::score::score, types::InvertedIndex, utils::display_top_results
};

fn main() {
    let mut inverted_index = InvertedIndex::new();
    commander("reindex".to_string(), Vec::new(), &mut inverted_index);

    loop {
        println!("");
        println!("🔍 Enter your Query:");
        let query = {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().to_string()
        };
        if query.starts_with("::") {
            let command_line = query.strip_prefix("::").unwrap_or("help");
            let mut parts = command_line.split_whitespace();
            let command = parts.next().unwrap_or("help");
            let args: Vec<&str> = parts.collect();

            let should_continue = commander(command.to_lowercase(), args, &mut inverted_index);
            if !should_continue {
                break;
            }
            continue;
        }
        println!("⏳ Searching...");
        let start_time = Instant::now();

        let scores = score(query, &inverted_index);

        let end_time = Instant::now();
        let duration = end_time.duration_since(start_time);

        display_top_results(scores, &inverted_index);
        println!("({:.2?}ms)", duration.as_millis());
    }
}
