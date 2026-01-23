use axum::{routing::get, Json, Router};
use midir::{MidiOutput, MidiOutputConnection};
use rand::random_range;
use serde::Serialize;
use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Serialize)]
struct Message {
    message: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route(
            "/",
            get(|| async {
                Json(Message {
                    message: format!("MidirBridge Controller Active: {}", random_range(1..=6)),
                })
            }),
        )
        .route("/play_one", get(|| play_midi_handler(0x90, "One")))
        .route("/play_two", get(|| play_midi_handler(0x91, "Two")))
        .route("/play_three", get(|| play_midi_handler(0x92, "Three")));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("MidirBridge, http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}

async fn play_midi_handler(status: u8, label: &'static str) -> Json<Message> {
    match run_midi(status).await {
        Ok(_) => Json(Message {
            message: format!("Play {} Success (Status: 0x{:X})", label, status),
        }),
        Err(e) => Json(Message {
            message: format!("Play {} ERROR: {}", label, e),
        }),
    }
}

async fn run_midi(status: u8) -> Result<(), Box<dyn Error>> {
    let midi_out = MidiOutput::new("Axum Midir Client")?;

    let out_ports = midi_out.ports();
    let midir_bridge_port = out_ports.iter().find(|p| {
        midi_out
            .port_name(p)
            .unwrap_or_default()
            .contains("MidirBridge")
    });

    match midir_bridge_port {
        Some(port) => {
            let mut conn = midi_out.connect(port, "midir-connection")?;

            play_chord(&mut conn, status).await?;
        }
        None => {
            return Err("Port MidirBridge not found".into());
        }
    }

    Ok(())
}

async fn play_chord(conn: &mut MidiOutputConnection, status: u8) -> Result<(), Box<dyn Error>> {
    let chord_notes = [60, 64, 67];
    let velocity = 100;

    for &note in &chord_notes {
        conn.send(&[status, note, velocity])?;
    }

    sleep(Duration::from_millis(1000)).await;

    let off_status = status - 0x10;

    for &note in &chord_notes {
        conn.send(&[off_status, note, 0])?;
    }

    Ok(())
}
