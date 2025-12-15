use crate::server::registry::ClientRegistry;
use anyhow::Result;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    net::TcpListener,
};
use uuid::Uuid;

pub async fn start_stream_server(
    addr: &str,
    registry: ClientRegistry,
) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;
    println!("Stream server listening on {}", addr);

    loop {
        let (stream, _) = listener.accept().await?;
        let registry = registry.clone();

        tokio::spawn(async move {
            if let Err(e) = handle_stream_client(stream, registry).await {
                eprintln!("Stream error: {}", e);
            }
        });
    }
}

async fn handle_stream_client(
    stream: tokio::net::TcpStream,
    registry: ClientRegistry,
) -> Result<()> {
    let mut reader = BufReader::new(stream);
    let mut line = String::new();

    // first message = session id
    reader.read_line(&mut line).await?;
    let msg: serde_json::Value = serde_json::from_str(&line)?;

    let session_id = Uuid::parse_str(msg["session_id"].as_str().unwrap())?;

    if !registry.lock().unwrap().contains_key(&session_id) {
        println!("❌ Invalid stream session {}", session_id);
        return Ok(());
    }

    println!("Stream connected: {}", session_id);

    // Now read raw data / frames
    loop {
        line.clear();
        let n = reader.read_line(&mut line).await?;
        if n == 0 {
            break;
        }

        // process stream data here
    }

    Ok(())
}