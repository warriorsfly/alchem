use std::{net::SocketAddr, sync::Arc};

use crate::handlers::{login_handler, signup_handler};
use axum::{
    routing::{get, post},
    Extension, Router, Server,
};

use alchem_websocket::{init_socket_server, ws_handler};
use handlers::{create_room_handler, join_room_handler, create_room_handler_2};

mod handlers;

#[tokio::main]
async fn main() {
    let serv = Arc::new(init_socket_server().await);
    let app = Router::new()
        // .fallback(
        //     get_service(
        //         ServeDir::new("./assets").append_index_html_on_directories(true),
        //     )
        //     .handle_error(|error: std::io::Error| async move {
        //         (
        //             StatusCode::INTERNAL_SERVER_ERROR,
        //             format!("Unhandled internal error: {}", error),
        //         )
        //     }),
        // )
        .route("/api/user/signup", post(signup_handler))
        .route("/api/user/login", post(login_handler))
        .route("/api/room", post(create_room_handler))
        .route("/api/room2", post(create_room_handler_2))
        .route("/api/room/join/:room_id", post(join_room_handler))
        .route("/ws", get(ws_handler))
        .layer(Extension(serv));

    // run it with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
