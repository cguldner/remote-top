//! Binary that receives the system information from the client and
//! displays it.
use std::net::{Ipv4Addr, SocketAddr};

use axum::{Json, Router, routing::post};
use remote_top::{SystemInformation, create_command_line_arg_parser};

async fn receive_json(Json(payload): Json<SystemInformation>) -> axum::http::StatusCode {
    println!("Server received: {:?}", payload);
    axum::http::StatusCode::OK
}

async fn shutdown_signal() {
    // Wait for Ctrl+C signal
    if let Err(e) = tokio::signal::ctrl_c().await {
        eprintln!("Failed to listen for shutdown signal: {}", e);
    }
    println!("\nReceived Ctrl+C, shutting down gracefully...");
}

#[tokio::main]
async fn main() {
    let _args = create_command_line_arg_parser(
        "Server that will receive system utilization information and display it on the screen"
            .to_string(),
    );

    // build our application with a single route
    let app = Router::new().route("/post", post(receive_json));
    // Run server in background task on localhost:3000
    let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, 3000));

    println!("Server listening on http://{}", addr);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}
