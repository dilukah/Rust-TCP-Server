use tcp_server_rust::server::{create_registry, listener::{ServerCallbacks, start_server}};
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use serde_json::json;

#[tokio::test]
async fn test_auth_success() {
    let registry = create_registry();
    let callbacks = ServerCallbacks::default();
    let addr = "127.0.0.1:9003";

    // Spawn the server in the background
    let server_handle = tokio::spawn({
        let registry = registry.clone();
        let callbacks = callbacks.clone();
        async move {
            start_server(addr, registry, callbacks).await.unwrap();
        }
    });

    // Wait until server is likely listening (simple, not perfect)
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    // Connect a client
    let mut stream = TcpStream::connect(addr).await.unwrap();

    // Load the token for authentication
    let token = tcp_server_rust::server::auth::load_or_create_token();

    // Send handshake JSON with newline
    let handshake = json!({
        "name": "Test",
        "role": "tester",
        "token": token
    });
    stream.write_all(format!("{}\n", handshake).as_bytes()).await.unwrap();

    // Optional: send a message
    let message = "hello server\n";
    stream.write_all(message.as_bytes()).await.unwrap();

    // Keep the connection briefly to ensure server handles it
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    // Close the client
    drop(stream);

    // Optional: shutdown the server task (or let it run)
    server_handle.abort();
}