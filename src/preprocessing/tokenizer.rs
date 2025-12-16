use std::{collections::HashSet, sync::OnceLock};

static STOPWORDS_SET: OnceLock<HashSet<String>> = OnceLock::new();
const STOPWORDS_PATH: &str = "stopwords.txt";

pub fn tokenize<T: AsRef<str>>(term: T) -> Option<String> {
    let mut token = term.as_ref().to_lowercase();
    token = remove_specials(token);

    if token.is_empty() {
        return None;
    }

    if is_stopword(&token) {
        return None;
    }

    token = stem(token);
    Some(token)
}

fn is_stopword(token: &String) -> bool {
    get_stopwords_set().contains(token)
}

fn stem(token: String) -> String {
    let mut token = token;
    token = remove_plural(token);
    token = remove_affix(token);
    token = remove_double_letters(token);
    token 
}

fn remove_specials(token: String) -> String {
    let mut token = token;

    token.retain(|c| c.is_alphanumeric());
    token
}

fn remove_plural(token: String) -> String {
    let len = token.len();
    if token.ends_with("ies") && len > 4 {
        return token[0..len - 3].to_owned() + "y";
    }
    if token.ends_with("es") && len > 3 {
        return token[0..len - 2].to_owned();
    }
    if token.ends_with("s") && len > 2 {
        return token[0..len - 1].to_owned();
    }
    token
}

fn remove_affix(token: String) -> String {
    let len = token.len();
    if token.ends_with("ing") && len > 3 {
        return token[0..len - 3].to_owned();
    }
    if token.ends_with("ed") && len > 2 {
        return token[0..len - 2].to_owned();
    }
    if token.ends_with("ly") && len > 2 {
        return token[0..len - 2].to_owned();
    }
    
    token
}

fn remove_double_letters(token: String) -> String {
    let len = token.len();
    if len >= 2 {
        let last_char = token.chars().last().unwrap();
        let second_to_last_char = token.chars().nth(len - 2).unwrap();

        if last_char == second_to_last_char {
            return token[0..len - 1].to_owned();
        }
    }

    token
}

fn get_stopwords_set() -> &'static HashSet<String> {
    STOPWORDS_SET.get_or_init(|| {
        let contents = std::fs::read_to_string(STOPWORDS_PATH)
            .expect("Failed to read stopwords file. Maybe the file is missing?");

        contents
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.trim().to_lowercase())
            .collect()
    })
}

