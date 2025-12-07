use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::net::SocketAddr;

// handshake from client (future expansion)
#[derive(Debug, Deserialize, Clone)]
pub struct Handshake {
    pub role: String,
    pub name: Option<String>,
}

// visible client info for server + GUI
#[derive(Debug, Clone, Serialize)]
pub struct ClientInfo {
    pub id: Uuid,
    pub name: String,
    pub role: String,
    pub addr: SocketAddr,
}