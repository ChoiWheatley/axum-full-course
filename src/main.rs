use std::net::SocketAddr;

use axum::{response::Html, Router};

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/hello",
        axum::routing::get(|| async {
            eprintln!("@@@@@ REQUEST RECEIVED!!!");
            Html("Hello <b>World!</b>")
        }),
    );
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("@@@@@ LISTENING ON {}", addr);
    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();
}
