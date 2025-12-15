pub type DocId = u32;
pub type Term = String;

mod inverted_index;

pub use inverted_index::Posting;
pub use inverted_index::InvertedIndex;
