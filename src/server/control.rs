use crate::server::{
    auth::load_or_create_token,
    client::{ClientInfo, Handshake},
    registry::ClientRegistry,
};

use anyhow::Result;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};
use uuid::Uuid;

pub async fn start_control_server(
    addr: &str,
    registry: ClientRegistry,
) -> Result<()> {
    let token = load_or_create_token();
    let listener = TcpListener::bind(addr).await?;

    println!("Control server (TLS) listening on {}", addr);

    loop {
        let (stream, addr) = listener.accept().await?;
        let registry = registry.clone();
        let token = token.clone();

        tokio::spawn(async move {
            if let Err(e) = handle_control_client(stream, addr.to_string(), registry, token).await {
                eprintln!("Control error: {}", e);
            }
        });
    }
}

async fn handle_control_client(
    stream: tokio::net::TcpStream, // will become TlsStream later
    addr: String,
    registry: ClientRegistry,
    server_token: String,
) -> Result<()> {
    let mut reader = BufReader::new(stream);
    let mut line = String::new();

    reader.read_line(&mut line).await?;
    let hs: Handshake = serde_json::from_str(&line)?;

    // --- AUTH ---
    if hs.token != server_token {
        println!("❌ Unauthorized control connection from {}", addr);
        return Ok(());
    }

    let session_id = Uuid::new_v4();
    let name = hs.name.unwrap_or_else(|| session_id.to_string());

    let client = ClientInfo {
        id: session_id,
        role: hs.role,
        name: name.clone(),
        addr,
    };

    registry.lock().unwrap().insert(session_id, client);

    println!("🟢 Control connected: {}", name);

    // respond with session info
    let response = serde_json::json!({
        "status": "ok",
        "session_id": session_id,
        "stream_port": 9001
    });

    reader
        .get_mut()
        .write_all(format!("{}\n", response).as_bytes())
        .await?;

    Ok(())
}