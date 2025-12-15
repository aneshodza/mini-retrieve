use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
};

use crate::types::DocId;

pub fn split_documents(doc_path: &str) {
    println!("> Splitting documents at path: {}", doc_path);
    let out_path = "out/documents";

    let file = File::open(doc_path).unwrap();
    println!("  > Found infile");

    let mut file_buffer: Vec<String> = vec![];
    let mut current_id: DocId = 0;
    let mut count: u32 = 0;

    let reader = BufReader::new(file);
    for line in reader.lines() {
        let text_line = line.unwrap();

        if text_line.starts_with(".I") {
            if file_buffer.len() > 0 {
                write_file(out_path, &file_buffer, current_id);
                file_buffer.clear();
            }

            let parsed_id_result = text_line
                .split_whitespace()
                .nth(1)
                .and_then(|s| s.parse::<DocId>().ok());
            if let Some(doc_id) = parsed_id_result {
                current_id = doc_id;
                count += 1;
            } else {
                eprintln!("WARNING: Failed to exctract a valid Doc ID. Line will be skipped.")
            }
        }
        file_buffer.push(text_line);
    }
    write_file(out_path, &file_buffer, current_id);
    println!("  > Document splitting completed");
    println!("  > Created {} documents under {}", count, out_path);
}

fn write_file(out_path: &str, file_buffer: &Vec<String>, current_id: DocId) {
    let content = file_buffer.join("\n");
    let out_path = format!("{}/doc{}", out_path, current_id);

    fs::write(out_path, content).unwrap_or_else(|e| {
        eprintln!("ERROR: Failed to write document {}", current_id);
        eprintln!("I/O Error details: {}", e);
    });
}
