use std::fs::File;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://picsum.photos/id/96/300/300";

    let response = reqwest::get(url).await?;
    let bytes = response.bytes().await?;

    let mut file = File::create("image.jpg").await?;
    file.write_all(&bytes).await?;

    println!("Image downloaded!");
    Ok(())
}
