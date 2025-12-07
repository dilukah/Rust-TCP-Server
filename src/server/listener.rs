use crate::server::{ClientRegistry, client::{ClientInfo, Handshake}};
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
    let listener = TcpListener::bind(addr).await?;
    println!("Server listening on {}", addr);

    loop {
        let (stream, addr) = listener.accept().await?;
        let registry = registry.clone();
        let cb = cb.clone();

        tokio::spawn(async move {
            if let Err(e) = handle_client(stream, addr.to_string(), registry, cb).await {
                eprintln!("Client error: {}", e);
            }
        });
    }
}

/// Handle individual client connection
async fn handle_client(stream: TcpStream, addr: String, registry: ClientRegistry, cb: ServerCallbacks) -> Result<()> {
    let mut reader = BufReader::new(stream);
    let mut line = String::new();

    // Expect handshake JSON
    reader.read_line(&mut line).await?;
    let hs: Handshake = serde_json::from_str(&line.trim())?;

    let id = Uuid::new_v4();
    let name = hs.name.unwrap_or_else(|| id.to_string());

    let client = ClientInfo { id, role: hs.role, name: name.clone(), addr };

    // store client
    {
        let mut reg = registry.lock().unwrap();
        reg.insert(id, client.clone());
    }
    if let Some(f) = cb.on_connect { f(client.clone()); }

    println!("🟢 {} connected", client.name);

    // Read loop
    let mut buf = String::new();
    loop {
        buf.clear();
        let n = reader.read_line(&mut buf).await?;
        if n == 0 { break; }

        if let Some(f) = cb.on_message { f(id, buf.clone()); }
        println!("[{}] {}", client.name, buf.trim());
    }

    // remove client
    {
        let mut reg = registry.lock().unwrap();
        reg.remove(&id);
    }
    if let Some(f) = cb.on_disconnect { f(id); }

    println!("🔴 {} left", name);
    Ok(())
}