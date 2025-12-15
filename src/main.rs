use tcp_server_rust::server::{
    create_registry,
    control::start_control_server,
    stream::start_stream_server,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let registry = create_registry();

    // Control server (TLS, auth, parameters)
    let control_registry = registry.clone();
    tokio::spawn(async move {
        start_control_server("127.0.0.1:9000", control_registry)
            .await
            .expect("control server failed");
    });

    // Stream server (plain TCP, high bandwidth)
    let stream_registry = registry.clone();
    tokio::spawn(async move {
        start_stream_server("127.0.0.1:9001", stream_registry)
            .await
            .expect("stream server failed");
    });

    println!("Control (TLS) on 9000, Stream (TCP) on 9001");

    // Keep main alive forever
    tokio::signal::ctrl_c().await?;
    Ok(())
}