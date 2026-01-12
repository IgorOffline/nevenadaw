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
        .route("/play_one", get(play_midi_handler_one))
        .route("/play_two", get(play_midi_handler_two));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on 0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn play_midi_handler_one() -> Json<Message> {
    match run_midi("ba954b8f").await {
        Ok(_) => Json(Message {
            message: format!("{} {}", "Play One", random_range(1..=6)),
        }),
        Err(_) => Json(Message {
            message: format!("{} {}", "Play One ERROR", random_range(1..=6)),
        }),
    }
}

async fn play_midi_handler_two() -> Json<Message> {
    match run_midi("e43c0181").await {
        Ok(_) => Json(Message {
            message: format!("{} {}", "Play Two", random_range(1..=6)),
        }),
        Err(_) => Json(Message {
            message: format!("{} {}", "Play Two ERROR", random_range(1..=6)),
        }),
    }
}

async fn run_midi(mode: &str) -> Result<(), Box<dyn Error>> {
    let midi_out = MidiOutput::new(&format!("{} {}", "My Test Output", mode))?;

    let out_ports = midi_out.ports();
    let midir_bridge_port = out_ports.iter().find(|p| {
        midi_out
            .port_name(p)
            .unwrap_or_default()
            .contains(&format!("{}{}", "MidirBridge", mode))
    });

    match midir_bridge_port {
        Some(port) => {
            let mut conn = midi_out.connect(port, &format!("{}{}", "midir-test-", mode))?;
            let status = match mode {
                "ba954b8f" => 0x90,
                "e43c0181" => 0x91,
                _ => return Err(format!("Invalid mode: {}", mode).into()),
            };
            play_chord(&mut conn, status).await?;
        }
        None => {
            return Err(format!("MidirBridge{} port not found", mode).into());
        }
    }

    Ok(())
}

async fn play_chord(conn: &mut MidiOutputConnection, status: u8) -> Result<(), Box<dyn Error>> {
    let chord_notes = [60, 64, 67];

    for &note in &chord_notes {
        let note_on = [status, note, 100];
        conn.send(&note_on)?;
    }

    sleep(Duration::from_millis(1000)).await;

    let off_status = status - 0x10;

    for &note in &chord_notes {
        let note_off = [off_status, note, 0];
        conn.send(&note_off)?;
    }

    Ok(())
}
