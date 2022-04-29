use std::{net::SocketAddr, sync::Arc};

use crate::handlers::{login_handler, signup_handler};
use axum::{
    routing::{get, post},
    Extension, Router, Server,
};

use alchem_websocket::{init_socket_server, ws_handler};

mod handlers;

#[tokio::main]
async fn main() {
    let sckt = Arc::new(init_socket_server().await);
    let app = Router::new()
        .route("/api/user/signup", post(signup_handler))
        .route("/api/user/login", post(login_handler))
        .route("/ws", get(ws_handler))
        .layer(Extension(sckt));

    // run it with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
