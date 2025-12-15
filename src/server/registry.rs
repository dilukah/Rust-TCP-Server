use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use uuid::Uuid;
use crate::server::client::ClientInfo;

pub type ClientRegistry = Arc<Mutex<HashMap<Uuid, ClientInfo>>>;

pub fn create_registry() -> ClientRegistry {
    Arc::new(Mutex::new(HashMap::new()))
}