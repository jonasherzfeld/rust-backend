use axum::{routing::{get, post}, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::{constants, errors::{Error, Result}};
use constants::LoginParams;
const AUTH_ROUTE: &str = "/auth";


pub fn routes() -> Router {
    Router::new()
        .route(&format!("{AUTH_ROUTE}/login"), post(api_login))
}

async fn api_login(payload: Json<LoginParams>) -> Result<Json<Value>> {
    println!("->> {:<12} - /api/login", "HANDLER");

    if payload.username != "admin" && payload.password != "admin" {
        return Err(Error::LoginFails);
    }

    let body = Json(json!({
        "result": {
            "success": true,
        }
    }));

    Ok(body)
}
