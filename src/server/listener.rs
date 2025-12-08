use crate::server::{ClientRegistry, client::{ClientInfo, Handshake}};
use crate::server::auth::load_or_create_token;

use anyhow::Result;
use tokio::{
    net::{TcpListener, TcpStream},
    io::{AsyncBufReadExt, BufReader},
};
use uuid::Uuid;

/// Callbacks allow plugging GUI or logging later
#[derive(Clone)]
pub struct ServerCallbacks {
    pub on_connect: Option<fn(ClientInfo)>,
    pub on_disconnect: Option<fn(uuid::Uuid)>,
    pub on_message: Option<fn(uuid::Uuid, String)>,
}

/// Run server
pub async fn start_server(addr: &str, registry: ClientRegistry, cb: ServerCallbacks) -> Result<()> {
    let token = load_or_create_token();  // load once
    let listener = TcpListener::bind(addr).await?;
    println!("Server listening on {}", addr);

    loop {
        let (stream, addr) = listener.accept().await?;
        let registry = registry.clone();
        let cb = cb.clone();
        let token = token.clone();  // clone for move into async task

        tokio::spawn(async move {
            if let Err(e) = handle_client(stream, addr.to_string(), registry, cb, token).await {
                eprintln!("Client error: {}", e);
            }
        });
    }
}

/// Handle individual client connection
async fn handle_client(
    stream: TcpStream,
    addr: String,
    registry: ClientRegistry,
    cb: ServerCallbacks,
    server_token: String,   
) -> Result<()> {
    let mut reader = BufReader::new(stream);
    let mut line = String::new();

    // read handshake
    reader.read_line(&mut line).await?;
    let hs: Handshake = serde_json::from_str(&line.trim())?;

    // ----------- AUTHENTICATION ----------- 
    // token check
    if hs.token != server_token {
    println!("❌ Unauthorized from {}", addr);
    return Ok(());
}

    // ----------- ACCEPT CLIENT ----------- 
    let id = Uuid::new_v4();
    let name = hs.name.unwrap_or_else(|| id.to_string());
    let client = ClientInfo { id, role: hs.role, name: name.clone(), addr };

    {
        let mut reg = registry.lock().unwrap();
        reg.insert(id, client.clone());
    }
    if let Some(f) = cb.on_connect { f(client.clone()); }
    println!("🟢 {} connected", client.name);

    // ----------- READ LOOP ----------- 
    let mut buf = String::new();
    loop {
        buf.clear();
        let n = reader.read_line(&mut buf).await?;
        if n == 0 { break; }

        if let Some(f) = cb.on_message { f(id, buf.clone()); }
        println!("[{}] {}", client.name, buf.trim());
    }

    // ----------- CLEANUP ----------- 
    {
        let mut reg = registry.lock().unwrap();
        reg.remove(&id);
    }
    if let Some(f) = cb.on_disconnect { f(id); }
    println!("🔴 {} left", name);

    Ok(())
}