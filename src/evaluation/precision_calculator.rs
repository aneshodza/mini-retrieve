use std::collections::{HashMap, HashSet};

use crate::{
    querying::score::score,
    types::{DocId, InvertedIndex},
};

pub fn mean_average_precision(queries: HashMap<u32, String>, inverted_index: &InvertedIndex) -> f32 {
    let mut running_sum: f32 = 0.;
    let qry_count = queries.len() as f32;

    for query in queries {
        let scores = score(query.1, inverted_index);
        running_sum += average_precision(query.0, scores);
    }

    running_sum / qry_count
}

fn average_precision(query: DocId, scores: HashMap<DocId, f32>) -> f32 {
    let (relevance_set, relevant_count) = get_relevance_set(query);

    if relevant_count == 0 {
        return 0.;
    }

    let mut found = 0;
    let mut running_sum: f32 = 0.;

    let mut ranked_results: Vec<(DocId, f32)> = scores.into_iter().collect();

    ranked_results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    for (rank, (doc_id, _score)) in ranked_results.iter().enumerate() {
        if relevance_set.contains(doc_id) {
            found += 1;
            running_sum += found as f32 / (rank + 1) as f32;

            if relevant_count == found {
                break;
            }
        }
    }

    running_sum / relevant_count as f32
}

fn get_relevance_set(query: DocId) -> (HashSet<DocId>, u32) {
    let qrel_path = "in/qrel";
    let file_content = std::fs::read_to_string(qrel_path).expect("Unable to read qrel file");

    let mut relevant_docs: HashSet<DocId> = HashSet::new();
    let mut relevant_count: u32 = 0;
    let query_str = query.to_string();

    for line in file_content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        let (curr_query_id_str, doc_id_str, grade_str) = match parts.as_slice() {
            [q, d, g] => (q, d, g),
            _ => continue,
        };

        if *curr_query_id_str != query_str {
            continue;
        }

        let doc_id: u32 = match doc_id_str.parse::<u32>() {
            Ok(d) => d,
            Err(_) => continue,
        };
        let grade: i32 = match grade_str.parse::<i32>() {
            Ok(g) => g,
            Err(_) => continue,
        };

        if grade >= 2 {
            relevant_count += 1;
            relevant_docs.insert(doc_id);
        }
    }
    (relevant_docs, relevant_count)
}
