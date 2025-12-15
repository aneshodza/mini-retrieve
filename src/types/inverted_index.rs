use std::collections::HashMap;

use crate::types::{DocId, Term};

#[derive(Debug)]
pub struct Posting {
    pub doc_id: DocId,
    pub tf: u32,
}

#[derive(Debug)]
pub struct InvertedIndex {
    pub dictionary: HashMap<Term, Vec<Posting>>,

    pub doc_lengths: HashMap<DocId, u32>,
    pub doc_titles: HashMap<DocId, String>,
    pub n: u32,
    pub avdl: f32,
}

impl Posting {
    pub fn new(doc_id: DocId, tf: u32) -> Self {
        Posting {
            doc_id: doc_id,
            tf: tf,
        }
    }
}

impl InvertedIndex {
    pub fn new() -> Self {
        InvertedIndex {
            dictionary: HashMap::new(),
            doc_lengths: HashMap::new(),
            doc_titles: HashMap::new(),
            n: 0,
            avdl: 0.0,
        }
    }

    pub fn add_posting(&mut self, term: Term, posting: Posting) {
        self.dictionary
            .entry(term)
            .or_insert_with(Vec::new)
            .push(posting);
    }
}
