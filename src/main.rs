#[macro_use]
extern crate diesel;

use alchem_websocket::AppState;
use axum::{
    async_trait,
    extract::{ FromRequest, RequestParts},
    http::StatusCode,
    routing::get,
     Router, Extension,
};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use std::{env, net::SocketAddr, time::Duration, sync::Arc};

#[tokio::main]
async fn main() {
    // Set the RUST_LOG, if it hasn't been explicitly defined
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "double-zero-raiser=debug")
    }
    tracing_subscriber::fmt::init();
    // setup connection pool
    let database_url = env::var("DATABASE_URL").expect("DATABASE_RUL is not set");
    let redis_urls = vec!["127.0.0.1"];
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let websocket_server = Arc::new(AppState::new(redis_urls));
    // build our application with some routes
    let app = Router::new()
        .layer(Extension(websocket_server))
        .layer(Extension(pool));

    // run it with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
