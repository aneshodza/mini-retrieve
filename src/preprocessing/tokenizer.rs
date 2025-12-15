use std::{collections::HashSet, sync::OnceLock};

static STOPWORDS_SET: OnceLock<HashSet<String>> = OnceLock::new();
const STOPWORDS: &[&str] = &[ // This is a random list i found in a GitHub Gist
    "i", "me", "my", "myself", "we", "our", "ours", "ourselves", "you", "your",
    "yours", "yourself", "yourselves", "he", "him", "his", "himself", "she", "her",
    "hers", "herself", "it", "its", "itself", "they", "them", "their", "theirs",
    "themselves", "what", "which", "who", "whom", "this", "that", "these", "those",
    "am", "is", "are", "was", "were", "be", "been", "being", "have", "has", "had",
    "having", "do", "does", "did", "doing", "a", "an", "the", "and", "but", "if",
    "or", "because", "as", "until", "while", "of", "at", "by", "for", "with", "about",
    "against", "between", "into", "through", "during", "before", "after", "above",
    "below", "to", "from", "up", "down", "in", "out", "on", "off", "over", "under",
    "again", "further", "then", "once", "here", "there", "when", "where", "why",
    "how", "all", "any", "both", "each", "few", "more", "most", "other", "some",
    "such", "no", "nor", "not", "only", "own", "same", "so", "than", "too", "very",
    "s", "t", "can", "will", "just", "don", "should", "now",
];

pub fn tokenize<T: AsRef<str>>(term: T) -> Option<String> {
    let mut token = term.as_ref().to_lowercase();
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
        STOPWORDS.into_iter().map(|s| s.to_string()).collect()
    })
}

