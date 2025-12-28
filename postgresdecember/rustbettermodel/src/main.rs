use ollama_rs::{
    Ollama, 
    generation::embeddings::request::{GenerateEmbeddingsRequest, EmbeddingsInput},
    generation::completion::request::GenerationRequest,
};
use tokio_postgres::NoTls;
use pgvector::Vector;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ollama = Ollama::new("http://localhost".to_string(), 11435);
    let db_url = "host=localhost port=5434 user=postgres dbname=vectordb password=postgres";
    
    let (client, conn) = tokio_postgres::connect(db_url, NoTls).await?;
    tokio::spawn(async move { if let Err(e) = conn.await { _ = e; } });

    let user_query = "a thrilling space adventure with robots";
    println!("d5324225 Searching for: \"{}\"...", user_query);

    let embed_req = GenerateEmbeddingsRequest::new("all-minilm".to_string(), EmbeddingsInput::Single(user_query.to_string()));
    let res = ollama.generate_embeddings(embed_req).await?;
    
    if let Some(raw_embedding) = res.embeddings.first() {
        let search_vector = Vector::from(raw_embedding.clone());

        let rows = client.query(
            "SELECT metadata->>'title', content FROM vector_store ORDER BY embedding <=> $1 LIMIT 3",
            &[&search_vector],
        ).await?;

        let mut context = String::new();
        for row in &rows {
            let title: &str = row.get(0);
            let desc: &str = row.get(1);
            context.push_str(&format!("Movie: {}\nDescription: {}\n---\n", title, desc));
        }

        let prompt = format!(
            "A user is looking for: '{}'. \
            Here are 3 movies from our database:\n{}\n \
            Based ONLY on these descriptions, which one is the best fit and why? \
            Answer in a friendly tone.",
            user_query, context
        );

        println!("d5324225 Llama 3.2 is thinking...\n");
        let gen_req = GenerationRequest::new("llama3.2:3b".to_string(), prompt);
        let gen_res = ollama.generate(gen_req).await?;

        println!("d5324225 Recommendation:\n{}", gen_res.response);
    }

    Ok(())
}