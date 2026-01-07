use ollama_rs::{
    Ollama, 
    generation::embeddings::request::{GenerateEmbeddingsRequest, EmbeddingsInput}
};
use tokio_postgres::NoTls;
use pgvector::Vector;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ollama = Ollama::new("http://localhost".to_string(), 11435);
    let db_url = "host=localhost port=5434 user=postgres dbname=vectordb password=postgres";
    
    let (client, conn) = tokio_postgres::connect(db_url, NoTls).await?;
    tokio::spawn(async move { if let Err(e) = conn.await { eprintln!("connection error: {}", e); } });

    let query_text = "a thrilling space adventure with robots";
    println!("c9984950 Searching for: \"{}\"...", query_text);

    let req = GenerateEmbeddingsRequest::new(
        "all-minilm".to_string(), 
        EmbeddingsInput::Single(query_text.to_string())
    );
    let res = ollama.generate_embeddings(req).await?;
    
    if let Some(raw_embedding) = res.embeddings.first() {
        let search_vector = Vector::from(raw_embedding.clone());

        let rows = client
            .query(
                "SELECT metadata->>'title' AS title, content 
                 FROM vector_store 
                 ORDER BY embedding <=> $1 
                 LIMIT 5",
                &[&search_vector],
            )
            .await?;

        println!();
        println!("c9984950 Top 5 Matches:");
        for (i, row) in rows.iter().enumerate() {
            let title: &str = row.get(0);
            let content: &str = row.get(1);
            println!("{}. {}: {}", i + 1, title, content);
        }
    }

    Ok(())
}