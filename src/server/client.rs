use serde::{Serialize, Deserialize};
use uuid::Uuid;

/// Metadata for each connected client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientInfo {
    pub id: Uuid,
    pub name: String,
    pub role: String,
    pub addr: String,
}

/// Incoming handshake expected from client
#[derive(Debug, Serialize, Deserialize)]
pub struct Handshake {
    pub name: Option<String>,
    pub role: String,
    pub token: String, 
}