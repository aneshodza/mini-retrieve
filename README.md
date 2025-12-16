# 🌟 Mini-Retrieve: A Blazing-Fast IR System in Rust

**Mini-Retrieve** is a high-performance, command-line Information Retrieval (IR) system built entirely in **Rust**. It serves as a fully functional, end-to-end implementation of a modern search engine core, specializing in fast indexing and highly relevant document ranking using the industry-standard BM25 algorithm.

The system is designed for speed, achieving near sub-millisecond query latency through Rust's efficiency and a clean index structure.

---

## 🚀 Features

Mini-Retrieve implements the complete IR pipeline from corpus ingestion to ranked results display:

### 📀 Indexing and Storage
* **Inverted Index:** Uses an efficient `HashMap` structure to store postings lists, which include Document ID (`DocId`) and Term Frequency (`tf`).
* **Metadata:** Stores essential global statistics (`n`, `avdl`) and per-document metadata (lengths and titles) within the `InvertedIndex` structure.
* **Document Handling:** Documents are processed from a tagged format (like Cranfield/CACM) by splitting them into individual files and extracting the document title (`.T` tag).

### 🪙 Tokenization
* **Special Character Removal**: Removes special characters from terms.
* **Stemming**: The terms get stemmed, as to merge similar terms using following methods:
    * **Remove Plural**: This removes certain plural suffixes (e.g. "ies" --> "y")
    * **Remove Affix**: This removes certain affixes (e.g. "ing" --> 🚫)
    * **Remove Double Letters**: This removes double-lettered suffixes which are often left over from affix removal (e.g. "runn" -> "run")
* **Stopword Filtering**: Certain terms with low informational value get removed.

### 🥇 Retrieval and Ranking
* **BM25 Algorithm:** Implements the Okapi BM25 ranking function for relevance scoring.
* **Query Processing:** All query terms are consistently tokenized and lowercased to ensure accurate matching against the index.
* **Performance Measurement:** The query time is measured using `std::time::Instant` and reported for benchmarking.

### 🏋️ User Interface
* **Interactive CLI:** Features an interactive command loop that supports queries and administrative commands prefixed with `::` (e.g., `::stats`, `::postings`).
* **Formatted Results:** Displays the top 10 search results in a clean, readable table including the document title and calculated BM25 score.

---

## 🛠 Getting Started

### Prerequisites

You need to have Rust installed on your system.

### Setup

1.  **Clone the Repository:**
    ```bash
    git clone git@github.com:aneshodza/mini-retrieve.git
    cd mini-retrieve
    ```

2.  **Add Corpus:** Place your document corpus file (in a tagged format like Cranfield) at the path: `./in/documents.all`.

3.  **Run the Application:**
    ```bash
    cargo run
    ```

The application will automatically execute the `::reindex` command on startup to process the corpus and build the inverted index.

---

## 💻 Usage

The system operates in an interactive loop. You can enter natural language queries or use administrative commands.

### Example Query
```
🔍 Enter your Query:
Information Retrieval Systems
⏳ Searching...

+------------------------------------------------------------------------------------------+
|                                    🎉 Top 10 Results                                     |
+----------+------------------------------------------------------------------+------------+
|  Doc ID  | Document Title                                                   |   Score    |
+----------+------------------------------------------------------------------+------------+
|   961    | compressible two dimensional jet mixing at constant              |    22.0917 |
|   798    | interaction between shock waves and boundary layers, with a note |    20.1529 |
|   1040   | on transverse vibrations of thin, shallow elastic shells         |    17.3793 |
|   1147   | heat transfer to bodies traveling at high speed in the upper     |    14.7801 |
|   1047   | the bending strength of pressurized cylinders                    |    13.9809 |
|   164    | an approximate analytical method for studying entry              |    13.8062 |
|   187    | investigation of separated flows in supersonic and subsonic      |    13.7083 |
|    96    | review of published data on the effect of roughness on transition|    13.4636 |
|   199    | measurement of two dimensional derivatives on a wing-aileron-tab |    13.3852 |
|   441    | evaluation of high angle-of-attack aerodynamic derivative        |    13.3820 |
+----------+------------------------------------------------------------------+------------+
(73.583µs)
```

### ⚙️ Administrative Commands

Commands must be prefixed with `::`.

| Command | Description |
| :--- | :--- |
| `::help` | Displays the list of available commands. |
| `::reindex` | Rebuilds the inverted index from the source corpus. |
| `::stats` | Shows index statistics (Total Docs, Total Terms, Avg. Doc Length). |
| `::postings <term>` | Displays the postings list (Doc ID, Title, TF) for a specific term. |
| `::doc <ID>` | Displays the raw content of a document by its ID. |
| `::eval` | Evaluates the Precision and recall |
| `::tokenize <terms>` | Tokenizes the sequence of terms passed and prints the result |
| `::exit` | Exits the application. |

---

## 🏎️ Running in Production Mode

For the best possible performance and the near sub-millisecond query latency this project is designed for, you should always run the compiled **release build**. This tells the Rust compiler to activate full optimization flags, significantly speeding up the indexing and the complex BM25 scoring calculations.

### 1. Build the Optimized Binary

Use the `--release` flag with Cargo:

```bash
cargo build --release
```
This process may take longer than a standard debug build, but the resulting executable will be highly optimized.

### 2. Execute the Production Binary

The compiled, optimized binary will be placed in the release subdirectory of your target folder.
```
./target/release/mini-retrieve
```

#### Note on Performance

When measuring performance using the `std::time::Instants` functionality included in `src/main.rs`, the most representative timing will be achieved using this production binary.
