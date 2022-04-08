use alchem_utils::{config::CONFIG, pool::init_pool, WsServer};
use axum::{routing::post, Extension, Router};

use std::net::SocketAddr;

use crate::handlers::signup;

mod handlers;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    // setup connection pool
    let redis_urls: Vec<&str> = CONFIG.redis_urls.split(',').collect();

    let pool = init_pool(&CONFIG.database_url);

    let key_pair = CONFIG.get_rsa();
    // build our application with some routes
    let app = Router::new()
        .layer(Extension(pool))
        // .layer(Extension(websocket_server))
        .layer(Extension(key_pair))
        .route("/api/signup", post(signup));

    // run it with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
