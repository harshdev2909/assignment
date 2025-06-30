use actix_web::{web, HttpResponse, Responder};
use crate::handlers::{keypair, token, message, sol};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::resource("/keypair").route(web::post().to(keypair::generate_keypair)))
        .service(web::resource("/token/create").route(web::post().to(token::create_token)))
        .service(web::resource("/token/mint").route(web::post().to(token::mint_token)))
        .service(web::resource("/message/sign").route(web::post().to(message::sign_message)))
        .service(web::resource("/message/verify").route(web::post().to(message::verify_message)))
        .service(web::resource("/send/sol").route(web::post().to(sol::send_sol)))
        .service(web::resource("/send/token").route(web::post().to(sol::send_token)));
}