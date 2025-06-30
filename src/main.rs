mod handlers;
mod models;
mod utils;

use axum::{routing::post, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/keypair", post(handlers::keypair::handle_keypair_generation))
        .route("/token/create", post(handlers::token::handle_token_creation))
        .route("/token/mint", post(handlers::token::handle_token_minting))
        .route("/send/token", post(handlers::token::handle_token_transfer))
        .route("/message/sign", post(handlers::message::handle_message_signing))
        .route("/message/verify", post(handlers::message::handle_message_verification))
        .route("/send/sol", post(handlers::sol::handle_sol_transfer))
        ;

    let tcp_listener = tokio::net::TcpListener::bind("0.0.0.0:3001")
        .await
        .expect("Failed to bind to port 3001");
    println!("Solana HTTP server running on http://0.0.0.0:3001");
    axum::serve(tcp_listener, app).await.unwrap();
}