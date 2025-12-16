use crate::{
    evaluation::{precision_calculator::mean_average_precision, query_extractor::extract_queries},
    preprocessing::{indexer, splitter, tokenizer::tokenize},
    querying::score::idf,
    types::InvertedIndex,
};
use std::{cmp::max, fs};
use std::{collections::HashSet, path::PathBuf};

pub fn commander(command: String, args: Vec<&str>, inverted_index: &mut InvertedIndex) -> bool {
    match command.as_str() {
        "exit" => exit(),
        "reindex" => index_build(inverted_index),
        "stats" => stats(inverted_index),
        "postings" => postings(args, inverted_index),
        "commons" => commons(inverted_index, args),
        "doc" => read_doc(args),
        "eval" => eval_queries(inverted_index, args),
        "tokenize" => print_tokenized(args),
        _ => print_help(),
    }
}

fn exit() -> bool {
    println!("👋 Bye!");
    false
}

fn index_build(inverted_index: &mut InvertedIndex) -> bool {
    let doc_path = "in/documents.all";
    println!("");
    splitter::split_documents(doc_path);
    println!("");
    *inverted_index = indexer::create_inverted_index();

    println!("");
    println!("> Preparations are done");
    println!("");
    println!("🤖 Mini Retrieve is ready to serve your queries!");
    println!("Type '::help' for available commands.");

    true
}

fn stats(inverted_index: &InvertedIndex) -> bool {
    println!("📊 Inverted Index Statistics:");
    println!("   Total Documents: {}", inverted_index.n);
    println!("   Total Terms: {}", inverted_index.dictionary.len());
    println!("   Average Document Length: {:.2}", inverted_index.avdl);

    true
}

fn postings(args: Vec<&str>, inverted_index: &InvertedIndex) -> bool {
    if args.is_empty() {
        println!("⚠️ Please provide a term to look up postings.");
    } else {
        let term = args[0];
        println!("\n🔤 Postings for Term: \"{}\"", term);
        let Some(token) = tokenize(term) else {
            println!("⚠️ You have entered a stopword. That doesn't have any postings.");
            return true;
        };

        if let Some(postings) = inverted_index.dictionary.get(&token) {
            println!("+--------+----------------------------------------------------+-------+");
            println!(
                "| {:<6} | {:<50} | {:<5} |",
                "Doc ID", "Document Title", "TF"
            );
            println!("+--------+----------------------------------------------------+-------+");

            for posting in postings {
                let doc_id = posting.doc_id;
                let tf = posting.tf;

                let title = inverted_index
                    .doc_titles
                    .get(&doc_id)
                    .map(|s| s.as_str())
                    .unwrap_or("!TITLE NOT FOUND!");

                let display_title = if title.len() > 50 {
                    format!("{:.47}...", title) // Truncate and add ellipsis
                } else {
                    title.to_string()
                };
                println!("| {:<6} | {:<50} | {:<5} |", doc_id, display_title, tf);
            }
            println!("+--------+----------------------------------------------------+-------+");
            println!(
                "ℹ️ Total occurrences found in {} documents.",
                postings.len()
            );
        } else {
            println!("Term \"{}\" was not found in the index dictionary.", term);
        }
    }
    true
}

fn commons(inverted_index: &InvertedIndex, args: Vec<&str>) -> bool {
    let mut count: u32 = 10;
    if !args.is_empty() {
        count = args[0].parse::<u32>().unwrap_or(10);
    }
    let mut term_stats: Vec<(&String, usize)> = inverted_index
        .dictionary
        .iter()
        .map(|(term, postings)| (term, postings.len()))
        .collect();

    term_stats.sort_by(|a, b| b.1.cmp(&a.1));

    let n = inverted_index.n;

    println!("+---------------------------------------------------------+");
    println!(
        "| 🏆 Top {:<3} Most Common Terms by Document Frequency (df) |",
        count
    );
    println!("+-------------------------+--------------------+----------+");
    println!(
        "| {:<23} | {:<18} | {:<8} |",
        "Term (Stem)", "Doc Frequency (df)", "IDF"
    );
    println!("+-------------------------+--------------------+----------+");

    for (term, df) in term_stats.iter().take(count as usize) {
        let idf_score = idf(*df as u32, n);
        let display_idf = format!("{:.4}", idf_score);

        println!("| {:<23} | {:<18} | {:<8} |", term, df, display_idf);
    }

    println!("+-------------------------+--------------------+----------+");

    println!("\nℹ️ Total Documents (N): {}", n);
    println!(
        "ℹ️ Average Document Length (AVDL): {:.2}",
        inverted_index.avdl
    );

    true
}

fn read_doc(args: Vec<&str>) -> bool {
    if args.is_empty() {
        println!("⚠️ Usage: ::doc <ID>");
        return true;
    }

    let doc_id_str = args[0];

    let mut doc_path = PathBuf::from("out/documents");
    doc_path.push(format!("doc{}", doc_id_str));

    println!(
        "📚 Attempting to read document from: '{}'",
        doc_path.display()
    );

    match fs::read_to_string(&doc_path) {
        Ok(content) => {
            let header_text = format!("📄 Document ID: {}", doc_id_str);
            let content_lines = content.lines();
            let max_content_width = content_lines
                .map(|line| line.chars().count())
                .max()
                .unwrap_or(0);
            let inner_width = max(header_text.chars().count(), max_content_width) + 2;

            let border_line = format!("+{}+", "-".repeat(inner_width));
            let footer_line = format!("+{}+", "-".repeat(inner_width));

            println!("\n{}", border_line);
            println!("| {:<width$} |", header_text, width = inner_width - 3);
            println!("{}", border_line);
            for line in content.lines() {
                println!("| {:<width$} |", line, width = inner_width - 2);
            }
            println!("{}\n", footer_line);
        }
        Err(e) => {
            println!("\n❌ Error reading document {}:", doc_id_str);
            println!("   File not found or could not be read. ({})", e);
        }
    }
    true
}

fn eval_queries(inverted_index: &InvertedIndex, args: Vec<&str>) -> bool {
    println!("🔬 Running evaluation...");
    let mut queries = extract_queries("in/documents.qry");
    if args.len() > 0 {
        let qids: HashSet<u32> = args
            .iter()
            .filter_map(|s| s.parse::<u32>().ok())
            .collect::<Vec<u32>>()
            .into_iter()
            .collect();

        println!(
            "ℹ️ Evaluating only queries with IDs: {:?}",
            qids.iter().collect::<Vec<&u32>>()
        );

        queries = queries
            .into_iter()
            .filter(|(query_id, _query_string)| qids.contains(query_id))
            .collect();
    } else {
        println!("ℹ️ Evaluating all queries as no specific IDs were provided.");
    }
    let map = mean_average_precision(queries, inverted_index);
    println!("\n🦀 The MAP was calculated to be: {}", map);

    true
}

fn print_tokenized(args: Vec<&str>) -> bool {
    let mut tokens: Vec<String> = Vec::new();
    for term in args {
        if let Some(token) = tokenize(term) {
            tokens.push(token);
        }
    }
    if tokens.len() > 0 {
        println!("🪙 Tokenized sequence: {}", tokens.join(" "));
    } else {
        println!("✂️ Sequence only had stopwords!");
    }
    true
}

fn print_help() -> bool {
    println!("📖 Available commands:");
    println!("   ::help                - Show this help message");
    println!("   ::exit                - Exit the application");
    println!("   ::reindex             - Rebuild the inverted index");
    println!("   ::stats               - Show statistics about the inverted index");
    println!("   ::postings <term>     - Show postings list for a term");
    println!("   ::commons <n>         - Shows n most common postings");
    println!("   ::doc <ID>            - Display the content of a document by its ID");
    println!("   ::eval                - Run predefined test queries with relevance list");
    println!("   ::tokenize <terms>    - Stem a sequence of terms");

    true
}
