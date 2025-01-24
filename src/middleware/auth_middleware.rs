use crate::utils::jwt::verify_jwt;
use axum::{
    extract::Request,
    http::{self, StatusCode},
    middleware::Next,
    response::{Json, Response},
};
use serde_json::{json, Value};

pub async fn verify_token(req: Request, next: Next) -> Result<Response, (StatusCode, Json<Value>)> {
    let token = match req.headers().get(http::header::AUTHORIZATION) {
        Some(value) => match value.to_str() {
            Ok(token_str) => token_str,
            Err(_) => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "error": "Invalid token format" })),
                ))
            }
        },
        None => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": "Missing authorization header" })),
            ))
        }
    };

    match verify_jwt(token) {
        Ok(_) => Ok(next.run(req).await),
        Err(_) => Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "error": "JWT token expired"
            })),
        )),
    }
}
