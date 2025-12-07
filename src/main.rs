mod server;

use server::{create_registry, listener::{start_server, ServerCallbacks}};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let registry = create_registry();

    let callbacks = ServerCallbacks {
        on_connect: Some(|c| println!("Client joined: {:?}", c)),
        on_disconnect: Some(|id| println!("Client left: {}", id)),
        on_message: Some(|id,msg| println!("Message from {} => {}", id,msg)),
    };

    start_server("127.0.0.1:9000", registry, callbacks).await
}