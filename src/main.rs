use std::net::SocketAddr;

pub use self::error::{Error, Result};

use axum::{
    extract::{Path, Query},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service, MethodRouter},
    Router,
};
use serde::Deserialize;
use tower_http::services::ServeDir;

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

mod error;
mod web;

#[tokio::main]
async fn main() {
    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .layer(middleware::map_response(main_response_mapper))
        .fallback_service(routes_static());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("@@@@@ LISTENING ON {}", addr);
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
}

/// special middleware response mapper
async fn main_response_mapper(res: Response) -> Response {
    println!("@@@@@ {:<12} - main_response_mapper", "RES_MAPPER");
    println!();

    res
}

/// Routes to our file system powered by `tower-http` crate.
/// Because roots cannot be overwrited, we have to `nest` that services
fn routes_static() -> MethodRouter {
    get_service(ServeDir::new("assets"))
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello2/:name", get(handler_hello2))
        .route("/hello", get(handler_hello))
}

/// e.g., `/hello?name=Jen`
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("@@@@@ {:<12} - {params:?}", "HANDLER");
    let name = params.name.unwrap_or("World".to_string());
    Html(format!("Hello <b>{name}!!!</b>"))
}

/// e.g., `/hello2/Mike`
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("@@@@@ {:<12} - {name:?}", "HANDLER");

    Html(format!("Hello <b>{name}!!!</b>"))
}
