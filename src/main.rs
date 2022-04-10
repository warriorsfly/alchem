use alchem_websocket::{ws_handler, Application};
use axum::{
    extract::Extension,
    routing::{get, post},
    Router, Server,
};

use std::{net::SocketAddr, sync::Arc};

use crate::handlers::{login_handler, signup_handler};

mod handlers;

#[tokio::main]
async fn main() {
    // tracing_subscriber::fmt::init();
    // let config = get_config();
    // let key_pair = Config::get_rsa(&config);
    // build our application with some routes
    let appli = Arc::new(Application::new());
    let app = Router::new()
        .layer(Extension(appli))
        .route("/api/user/signup", post(signup_handler))
        .route("/api/user/login", post(login_handler))
        .route("/ws", get(ws_handler));

    // run it with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
