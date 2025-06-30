// This file is the entry point of the application. It sets up the HTTP server and routes for handling incoming requests.

use actix_web::{web, App, HttpServer};

mod handlers;
pub mod models;
pub mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/keypair").route("", web::post().to(handlers::keypair::generate_keypair)))
            .service(web::scope("/token")
                .route("/create", web::post().to(handlers::token::create_token))
                .route("/mint", web::post().to(handlers::token::mint_token)))
            .service(web::scope("/message")
                .route("/sign", web::post().to(handlers::message::sign_message))
                .route("/verify", web::post().to(handlers::message::verify_message)))
            .service(web::scope("/send")
                .route("/sol", web::post().to(handlers::sol::send_sol))
                .route("/token", web::post().to(handlers::sol::send_token)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}