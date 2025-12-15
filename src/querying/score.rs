use std::collections::HashMap;

use crate::{
    preprocessing::tokenizer::tokenize,
    types::{DocId, InvertedIndex},
    utils::calculate_document_tf,
};

const K1: f32 = 1.2;
const B: f32 = 0.75;

pub fn score(query: String, inverted_index: &InvertedIndex) -> HashMap<u32, f32> {
    let (qtf_map, _query_length) = calculate_document_tf(&query);

    let n = inverted_index.n;
    let avdl = inverted_index.avdl;
    let mut scores: HashMap<DocId, f32> = HashMap::new();

    qtf_map.iter().for_each(|(term, _qtf)| {
        if let Some(token) = tokenize(term) {
            match inverted_index.dictionary.get(&token) {
                Some(postings) => {
                    let df_j = postings.len() as u32;
                    let idf_j = idf(df_j, n);

                    for posting in postings {
                        let doc_id = posting.doc_id;
                        let tf_ij = posting.tf as f32;
                        let l_di = inverted_index
                            .doc_lengths
                            .get(&doc_id)
                            .copied()
                            .unwrap_or(0) as f32;

                        let score = idf_j * score_component(tf_ij, l_di, avdl);
                        *scores.entry(doc_id).or_insert(0.0) += score;
                    }
                }
                None => {}
            }
        }
    });

    scores
}

fn score_component(tf_ij: f32, l_di: f32, avdl: f32) -> f32 {
    (K1 + 1.0) * tf_ij / tf_ij + K1 * (1.0 - B + B * l_di / avdl)
}

fn idf(df_j: u32, n: u32) -> f32 {
    let dividend = (n - df_j) as f32 + 0.5;
    let divisor = df_j as f32 + 0.5;

    (dividend / divisor).ln()
}
