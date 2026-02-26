use anyhow::Result;
use reqwest::blocking::Client;
use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use std::time::Duration;

#[derive(Serialize)]
struct OllamaEmbedReq<'a> {
    model: &'a str,
    prompt: &'a str,
}

#[derive(Deserialize)]
struct OllamaEmbedResp {
    embedding: Vec<f32>,
}

const TITLES: [&str; 12] = [
    "React state management patterns for shared components",
    "Using CSS Grid for responsive dashboard layouts",
    "Training a transformer on small datasets with regularization",
    "Vector embeddings enable semantic search over documents",
    "Kafka partitions and consumer groups for scalable throughput",
    "Optimizing Spark DataFrames with predicate pushdown",
    "Designing idempotent Airflow DAGs for reliability",
    "Docker multi-stage builds to keep images small",
    "Kubernetes Ingress vs Service: routing trade-offs",
    "Time-series downsampling with Rust and Polars",
    "Postgres: B-tree vs GIN indexes for text search",
    "RAG: chunking, embeddings, and prompt assembly",
];

fn embed_text(client: &Client, base: &str, model: &str, text: &str) -> Result<Vec<f32>> {
    let url = format!("{base}/api/embeddings");
    let req = OllamaEmbedReq {
        model,
        prompt: text,
    };
    let resp = client.post(&url).json(&req).send()?.error_for_status()?;
    let body: OllamaEmbedResp = resp.json()?;
    Ok(body.embedding)
}

fn main() -> Result<()> {
    let model = "mxbai-embed-large";
    let ollama_url = "http://localhost:11434";

    let http = Client::builder().timeout(Duration::from_secs(60)).build()?;

    let db = Connection::open_in_memory()?;

    // Load sqlite-vector extension from the dylib next to the binary
    let ext_path = std::env::current_dir()?.join("sqlite-vector");
    unsafe {
        db.load_extension_enable()?;
        db.load_extension(&ext_path, Some("sqlite3_vector_init"))?;
        db.load_extension_disable()?;
    }

    // Create a regular table and initialize the vector column
    db.execute_batch(
        r"
        CREATE TABLE articles (
            id INTEGER PRIMARY KEY,
            title TEXT,
            embedding BLOB
        );
        ",
    )?;
    // see https://github.com/sqliteai/sqlite-vector
    db.query_row(
        "SELECT vector_init('articles', 'embedding', 'dimension=1024,type=FLOAT32,distance=cosine')",
        [],
        |_| Ok(()),
    )?;

    println!("Embedding {n} titles with {model}...", n = TITLES.len());

    for s in TITLES.iter() {
        let embedding = embed_text(&http, ollama_url, model, s)?;
        anyhow::ensure!(
            embedding.len() == 1024,
            "embedding dimension changed: {}",
            embedding.len()
        );
        let json = serde_json::to_string(&embedding)?;
        db.execute(
            "INSERT INTO articles (title, embedding) VALUES (?1, vector_as_f32(?2))",
            params![s, json],
        )?;
    }
    println!("Ready! Type a search query (or 'quit' to exit).\n");

    let stdin = io::stdin();

    loop {
        print!("search> ");
        io::stdout().flush()?;

        let mut query = String::new();
        if stdin.read_line(&mut query)? == 0 {
            break; // EOF
        }
        let query = query.trim();
        if query.is_empty() {
            continue;
        }
        if query == "quit" || query == "exit" {
            break;
        }

        let embedding = embed_text(&http, ollama_url, model, query)?;
        let json = serde_json::to_string(&embedding)?;

        println!();
        let mut stmt = db.prepare(
            "SELECT a.title, v.distance
               FROM vector_full_scan('articles', 'embedding', vector_as_f32(?1), 5) AS v
               JOIN articles AS a ON a.rowid = v.rowid",
        )?;
        let mut rows = stmt.query([&json])?;
        let mut rank = 1;
        while let Some(r) = rows.next()? {
            let title: String = r.get(0)?;
            let dist: f64 = r.get(1)?;
            println!("{rank:>2}. d={dist:.3}  {title}");
            rank += 1;
        }
        println!();
    }

    Ok(())
}
