/// define the **payload** of the login what will be sent from the client.
use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::{Error, Result};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

/// handler for login routes
/// returns our typed `Result` where our `Error` type had implemented `IntoResponse`
async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("@@@@@ {:<12} - api_login", "HANDLER");

    // TODO: Implement real database and authentication logic
    if payload.username != "demo1" || payload.pwd != "welcome" {
        return Err(Error::LoginFail);
    }

    // TODO: Set cookies

    // Create the success body
    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}
