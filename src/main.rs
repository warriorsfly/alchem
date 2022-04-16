use axum::{
    routing::{get, post},
    Extension, Router, Server,
};
use server::{ws_handler, SocketServer};

use crate::handlers::{login_handler, signup_handler};
use std::{net::SocketAddr, sync::Arc};

mod handlers;
mod server;

#[tokio::main]
async fn main() {
    // tracing_subscriber::fmt::init();
    // let config = get_config();
    // let key_pair = Config::get_rsa(&config);
    // build our application with some routes
    let dim = Arc::new(SocketServer::new());
    let app = Router::new()
        .route("/api/user/signup", post(signup_handler))
        .route("/api/user/login", post(login_handler))
        .route("/ws", get(ws_handler))
        .layer(Extension(dim));

    // run it with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
