use std::collections::HashMap;

use crate::types::{DocId, InvertedIndex, Term};

const MAX_TITLE_WIDTH: usize = 70;

pub fn calculate_document_tf(content: &str) -> (HashMap<Term, u32>, u32) {
    let mut doc_length: u32 = 0;
    let mut tf_map: HashMap<Term, u32> = HashMap::new();

    content.split_whitespace().for_each(|token| {
        doc_length += 1;
        let normalized_token = token.to_lowercase();
        let counter = tf_map.entry(normalized_token).or_insert(0);
        *counter += 1;
    });

    (tf_map, doc_length)
}

pub fn display_top_results(scores: HashMap<DocId, f32>, index: &InvertedIndex) {
    let mut ranked_results: Vec<(DocId, f32)> = scores.into_iter().collect();

    ranked_results.sort_by(|a, b| {
        b.1.partial_cmp(&a.1)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    println!("\n+------------------------------------------------------------------------------------------------+");
    println!("|                                       🎉 Top 10 Results                                        |");
    println!("+----------+------------------------------------------------------------------------+------------+");
    println!(
        "| {:^8} | {:<MAX_TITLE_WIDTH$} | {:^10} |",
        "Doc ID", "Document Title", "Score"
    );
    println!("+----------+------------------------------------------------------------------------+------------+");

    for (rank, (doc_id, score)) in ranked_results.iter().take(10).enumerate() {
        let title = index
            .doc_titles
            .get(doc_id)
            .map(|s| s.as_str())
            .unwrap_or("Title Missing");

        let truncated_title = if title.len() > MAX_TITLE_WIDTH {
            format!("{}...", &title[0..MAX_TITLE_WIDTH - 3])
        } else {
            title.to_string()
        };

        println!(
            "| {:^8} | {:<MAX_TITLE_WIDTH$} | {:>10.4} |",
            doc_id,
            truncated_title,
            score
        );
    }
    println!("+----------+------------------------------------------------------------------------+------------+");
}

pub fn extract_title_from_content(content: &str) -> String {
    let mut lines = content.lines();

    while let Some(line) = lines.next() {
        if line.trim().starts_with(".T") {
            if let Some(title_line) = lines.next() {
                return title_line.trim().trim_end_matches('.').to_string();
            }
        }
    }
    "Title Missing".to_string()
}
