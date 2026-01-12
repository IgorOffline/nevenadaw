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
                    message: format!("{} {}", "Third Midiex", random_range(1..=6)),
                })
            }),
        )
        .route("/play", get(play_midi_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on 0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn play_midi_handler() -> Json<Message> {
    match run_midi().await {
        Ok(_) => Json(Message {
            message: format!("{} {}", "Play", random_range(1..=6)),
        }),
        Err(_) => Json(Message {
            message: format!("{} {}", "Play ERROR", random_range(1..=6)),
        }),
    }
}

async fn run_midi() -> Result<(), Box<dyn Error>> {
    let midi_out = MidiOutput::new("My Test Output")?;

    let out_ports = midi_out.ports();
    let midir_bridge_port = out_ports.iter().find(|p| {
        midi_out
            .port_name(p)
            .unwrap_or_default()
            .contains("MidirBridge")
    });

    match midir_bridge_port {
        Some(port) => {
            let mut conn = midi_out.connect(port, "midir-test")?;
            play_chord(&mut conn).await?;
        }
        None => {
            return Err("MidirBridge port not found".into());
        }
    }

    Ok(())
}

async fn play_chord(conn: &mut MidiOutputConnection) -> Result<(), Box<dyn Error>> {
    let chord_notes = [60, 64, 67];

    for &note in &chord_notes {
        let note_on = [0x90, note, 100];
        conn.send(&note_on)?;
    }

    sleep(Duration::from_millis(1000)).await;

    for &note in &chord_notes {
        let note_off = [0x80, note, 0];
        conn.send(&note_off)?;
    }

    Ok(())
}
