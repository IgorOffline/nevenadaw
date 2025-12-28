use ollama_rs::{
    Ollama, 
    generation::embeddings::request::{GenerateEmbeddingsRequest, EmbeddingsInput}
};
use tokio_postgres::NoTls;
use pgvector::Vector;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ollama = Ollama::new("http://localhost".to_string(), 11435);
    let db_url = "host=localhost port=5434 user=postgres password=postgres";
    
    let (client_rental, conn_rental) = tokio_postgres::connect(&format!("{} dbname=dvdrental", db_url), NoTls).await?;
    let (client_vector, conn_vector) = tokio_postgres::connect(&format!("{} dbname=vectordb", db_url), NoTls).await?;

    tokio::spawn(async move { if let Err(e) = conn_rental.await { _ = e; } });
    tokio::spawn(async move { if let Err(e) = conn_vector.await { _ = e; } });

    println!("a1de7717 Starting semantic sync...");

    let rows = client_rental.query("SELECT title, description FROM film", &[]).await?;

    for row in rows {
        let title: String = row.get(0);
        let description: String = row.get(1);

        let req = GenerateEmbeddingsRequest::new(
            "all-minilm".to_string(), 
            EmbeddingsInput::Single(description.clone())
        );
        
        let res = ollama.generate_embeddings(req).await?;
        
        if let Some(raw_embedding) = res.embeddings.first() {
            let embedding = Vector::from(raw_embedding.clone());
            let metadata = serde_json::json!({"title": title});
            
            client_vector
                .execute(
                    "INSERT INTO vector_store (content, metadata, embedding) VALUES ($1, $2, $3)",
                    &[&description, &metadata, &embedding],
                )
                .await?;
                
            println!("a1de7717 Embedded: {}", title);
        }
    }

    println!("a1de7717 Sync complete!");
    Ok(())
}