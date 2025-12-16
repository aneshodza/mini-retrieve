use std::{collections::HashMap, fs, path::Path};

use crate::{
    preprocessing::tokenizer::tokenize,
    types::{DocId, InvertedIndex, Posting, Term},
    utils::{calculate_document_tf, extract_title_from_content},
};

pub fn create_inverted_index() -> InvertedIndex {
    println!("> Creating inverted index");

    let mut inverted_index = InvertedIndex::new();
    println!("  > Inverted index created");

    let mut doc_lengths: HashMap<DocId, u32> = HashMap::new();
    let mut doc_titles: HashMap<DocId, String> = HashMap::new();
    let mut doc_count: u32 = 0;
    let mut term_count: u32 = 0;

    println!("   > Filling inverted index");
    let documents_dir = Path::new("out/documents");
    match fs::read_dir(documents_dir) {
        Ok(entries) => {
            for entry in entries {
                let entry = entry.unwrap_or_else(|_e| {
                    panic!("ERROR: Could not read document.");
                });

                let path = entry.path();
                if path.is_file() && path.file_name().is_some() {
                    let filename = path
                        .file_name()
                        .and_then(|name| name.to_str())
                        .unwrap_or("");

                    if !filename.starts_with("doc") {
                        continue;
                    }

                    doc_count += 1;
                    let doc_id = extract_doc_id(filename);

                    let content = fs::read_to_string(&path)
                        .unwrap_or_else(|_e| panic!("ERROR: Could not read document."));

                    let title = extract_title_from_content(&content);
                    doc_titles.insert(doc_id, title);

                    let (tf_map, doc_length) = calculate_document_tf(&content);
                    term_count += doc_length;
                    doc_lengths.insert(doc_id, doc_length);
                    update_inverted_index(&mut inverted_index, doc_id, tf_map);
                }
            }
        }
        Err(e) => {
            panic!("FATAL: Failed to read documents directory. Error: {}", e);
        }
    }

    inverted_index.n = doc_count;
    inverted_index.avdl = term_count as f32 / doc_count as f32;
    inverted_index.doc_lengths = doc_lengths;
    inverted_index.doc_titles = doc_titles;
    println!("  > Inverted index filled");

    inverted_index
}

fn extract_doc_id(filename: &str) -> DocId {
    filename
        .strip_prefix("doc")
        .and_then(|s| s.parse::<DocId>().ok())
        .unwrap_or_else(|| panic!("FATAL: Could not parse DocId from filename: {:?}", filename))
}

fn update_inverted_index(
    inverted_index: &mut InvertedIndex,
    doc_id: DocId,
    tf_map: HashMap<Term, u32>,
) {
    tf_map.into_iter().for_each(|(term, tf)| {
        if let Some(token) = tokenize(term) {
            inverted_index.add_posting(token, Posting::new(doc_id, tf));
        }
    });
}
