use anyhow::{bail, Context, Result};
use axum::Router;
use clap::Parser;
use regex::Regex;
use std::{
    fs,
    path::{Path, PathBuf},
    sync::OnceLock,
};
use tower_http::services::{ServeDir, ServeFile};

#[derive(Parser, Debug)]
#[command(
    name = "marchreadalong",
    version,
    about = "Generate a readalong HTML from assets/<name>.txt + assets/<name>.vtt + audio, using assets/template.html"
)]
struct Cli {
    /// Base name (stem). You may also pass a filename like foo.txt; the extension will be stripped.
    name: String,

    /// Assets directory (default: ./assets)
    #[arg(long, default_value = "assets")]
    assets_dir: PathBuf,

    /// Template path (default: <assets_dir>/template.html)
    #[arg(long)]
    template: Option<PathBuf>,

    /// Output HTML path (default: ./readalong.html)
    #[arg(long, default_value = "readalong.html")]
    output: PathBuf,

    /// Address to serve on (default: 127.0.0.1:3000)
    #[arg(long, default_value = "127.0.0.1:3000")]
    addr: String,
}

fn strip_known_ext(name: &str) -> &str {
    for ext in [".txt", ".vtt", ".wav", ".mp3"] {
        if let Some(stem) = name.strip_suffix(ext) {
            return stem;
        }
    }
    name
}

fn html_escape(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn split_sentences(text: &str) -> Vec<String> {
    static WS: OnceLock<Regex> = OnceLock::new();
    static BOUNDARY: OnceLock<Regex> = OnceLock::new();

    let ws = WS.get_or_init(|| Regex::new(r"\s+").expect("valid regex"));
    let boundary = BOUNDARY.get_or_init(|| Regex::new(r"([.!?]+)\s+").expect("valid regex"));

    let normalized = ws.replace_all(text, " ");
    let normalized = normalized.trim();

    if normalized.is_empty() {
        return Vec::new();
    }

    let mut out = Vec::new();
    let mut last = 0usize;

    for caps in boundary.captures_iter(normalized) {
        let m = caps.get(0).expect("match exists");
        let p = caps.get(1).expect("punctuation capture exists");

        let chunk = normalized[last..m.start()].trim();
        if !chunk.is_empty() {
            let mut sentence = String::with_capacity(chunk.len() + p.as_str().len());
            sentence.push_str(chunk);
            sentence.push_str(p.as_str());
            out.push(sentence);
        }

        last = m.end();
    }

    let tail = normalized[last..].trim();
    if !tail.is_empty() {
        out.push(tail.to_string());
    }

    out
}

fn ensure_exists(path: &Path, label: &str) -> Result<()> {
    if path.exists() {
        Ok(())
    } else {
        bail!("{label} not found: {}", path.display())
    }
}

fn pick_audio_path(assets_dir: &Path, stem: &str) -> Result<PathBuf> {
    let wav = assets_dir.join(format!("{stem}.wav"));
    if wav.exists() {
        return Ok(wav);
    }

    let mp3 = assets_dir.join(format!("{stem}.mp3"));
    if mp3.exists() {
        return Ok(mp3);
    }

    bail!(
        "Audio not found. Expected {} or {}",
        wav.display(),
        mp3.display()
    )
}

fn generate_html(
    template_path: &Path,
    stem: &str,
    assets_dir: &Path,
    output_path: &Path,
) -> Result<PathBuf> {
    let txt_path = assets_dir.join(format!("{stem}.txt"));
    let vtt_path = assets_dir.join(format!("{stem}.vtt"));
    let audio_path = pick_audio_path(assets_dir, stem)?;

    ensure_exists(assets_dir, "Assets directory")?;
    ensure_exists(&txt_path, "Text file")?;
    ensure_exists(&vtt_path, "VTT file")?;
    ensure_exists(template_path, "Template file")?;

    let txt = fs::read_to_string(&txt_path)
        .with_context(|| format!("Failed to read {}", txt_path.display()))?;

    let sentences = split_sentences(&txt);

    let mut spans_html = String::new();
    for (i, s) in sentences.iter().enumerate() {
        spans_html.push_str(&format!(
            r#"<span class="sent" id="s{idx}">{txt}</span> "#,
            idx = i,
            txt = html_escape(s)
        ));
    }

    let vtt_url = format!("/assets/{stem}.vtt");
    let audio_url = format!(
        "/assets/{}",
        audio_path
            .file_name()
            .context("Audio path had no filename")?
            .to_string_lossy()
    );

    let template = fs::read_to_string(template_path)
        .with_context(|| format!("Failed to read template {}", template_path.display()))?;

    let out = template
        .replace("{{AUDIO_FILE}}", &audio_url)
        .replace("{{VTT_FILE}}", &vtt_url)
        .replace("{{SPANS_HTML}}", &spans_html);

    fs::write(output_path, out)
        .with_context(|| format!("Failed to write output {}", output_path.display()))?;

    Ok(output_path.to_path_buf())
}

async fn serve(addr: &str, assets_dir: PathBuf, output_html: PathBuf) -> Result<()> {
    let app = Router::new()
        .nest_service("/assets", ServeDir::new(assets_dir))
        .fallback_service(ServeFile::new(output_html));

    println!("Serving on http://{addr}/");
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .with_context(|| format!("Failed to bind to {addr}"))?;

    axum::serve(listener, app).await.context("Server error")?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let stem = strip_known_ext(&cli.name).to_string();

    let template_path = cli
        .template
        .clone()
        .unwrap_or_else(|| cli.assets_dir.join("template.html"));

    let output_html = generate_html(&template_path, &stem, &cli.assets_dir, &cli.output)?;
    println!("Generated {}", output_html.display());

    serve(&cli.addr, cli.assets_dir, output_html).await
}
