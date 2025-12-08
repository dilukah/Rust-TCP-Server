pub mod listener;
pub mod client;
pub mod auth;

use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use uuid::Uuid;

use client::ClientInfo;

/// Shared registry type
pub type ClientRegistry = Arc<Mutex<HashMap<Uuid, ClientInfo>>>;

/// Public helper to build server
pub fn create_registry() -> ClientRegistry {
    Arc::new(Mutex::new(HashMap::new()))
}