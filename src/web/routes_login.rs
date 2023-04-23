/// define the **payload** of the login what will be sent from the client.
use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

use crate::{web::AUTH_TOKEN, Error, Result};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

/// handler for login routes
/// returns our typed `Result` where our `Error` type had implemented `IntoResponse`
async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("@@@@@ {:<12} - api_login", "HANDLER");

    // TODO: Implement real database and authentication logic
    if payload.username != "demo1" || payload.pwd != "welcome" {
        return Err(Error::LoginFail);
    }

    // cookie value format: {{username}}-{{user-id}}.{{expression date}}.{{signature}}
    // FIXME: Implement real auth-token generation and signature, which is out of this tutorial
    cookies.add(Cookie::new(AUTH_TOKEN, "user-1.exp.sign"));

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
