use crate::models::user::{NewUser, User};
use crate::schema::users::dsl::*;
use crate::utils::hash::verify_pw;
use crate::utils::{
    hash::hash_pw,
    jwt::{generate_jwt, verify_jwt, Claims},
};
use crate::AppState;
use axum::{
    extract::{self, State},
    http::StatusCode,
    response::{self, IntoResponse},
    routing::{get, post},
    Router,
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use diesel::prelude::*;
use serde::Deserialize;
use serde_json::json;

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .route("/logout", post(logout))
        .route("/user", get(get_user))
}

#[derive(Deserialize)]
struct RegisterRequest {
    email: String,
    password: String,
}

async fn register(
    State(state): State<AppState>,
    extract::Json(payload): extract::Json<RegisterRequest>,
) -> impl IntoResponse {
    let conn = &mut state
        .db_pool
        .get()
        .expect("Failed to get a connection from the pool");

    let user = match users.filter(email.eq(&payload.email)).first::<User>(conn) {
        Ok(_) => {
            return (
                StatusCode::CONFLICT,
                response::Json(json!({ "error": "User already exists" })),
            );
        }
        Err(diesel::result::Error::NotFound) => {
            let hashed_password = hash_pw(payload.password);
            NewUser {
                email: payload.email,
                password_hash: hashed_password,
            }
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                response::Json(json!({ "error": "Failed to query the database" })),
            );
        }
    };

    match diesel::insert_into(users).values(&user).execute(conn) {
        Ok(_) => {
            return (
                StatusCode::OK,
                response::Json(json!({ "message": "Registration successful." })),
            );
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                response::Json(json!({ "error": "User was unable to be saved to database." })),
            );
        }
    };
}

#[derive(Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    extract::Json(payload): extract::Json<LoginRequest>,
) -> impl IntoResponse {
    let conn = &mut state
        .db_pool
        .get()
        .expect("Failed to get a connection from the pool");

    let user = match users.filter(email.eq(&payload.email)).first::<User>(conn) {
        Ok(user) => user,
        Err(diesel::result::Error::NotFound) => {
            return (
                StatusCode::UNAUTHORIZED,
                jar,
                response::Json(json!({ "error": "Invalid email or password." })),
            );
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                jar,
                response::Json(json!({ "error": "Failed to query the database." })),
            );
        }
    };

    match verify_pw(payload.password, user.password_hash) {
        Ok(is_valid) => {
            if !is_valid {
                return (
                    StatusCode::UNAUTHORIZED,
                    jar,
                    response::Json(json!({ "error": "Invalid email or password." })),
                );
            }
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                jar,
                response::Json(json!({ "error": "Failed to verify password." })),
            );
        }
    }

    let token = match generate_jwt(user.user_id) {
        Ok(token) => token,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                jar,
                response::Json(json!({ "error": "Failed to generate JWT." })),
            );
        }
    };

    let cookie = Cookie::build(("auth_token", token))
        .http_only(true)
        .secure(false)
        .path("/")
        .max_age(time::Duration::days(1));

    let updated_jar = jar.add(cookie);

    return (
        StatusCode::OK,
        updated_jar,
        response::Json(json!({
            "message": "Login successful.",
            "user": {
                "user_id": user.user_id,
                "email": user.email
            }
        })),
    );
}

async fn logout() -> impl IntoResponse {
    return (
        StatusCode::OK,
        response::Json(json!({
            "message": "Successful logout"
        })),
    );
}

async fn get_user(State(state): State<AppState>, jar: CookieJar) -> impl IntoResponse {
    let conn = &mut state
        .db_pool
        .get()
        .expect("Failed to get a connection from the pool");

    let token = match jar.get("auth_token") {
        Some(cookie) => cookie.value().to_string(),
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                response::Json(json!({ "error": "Missing auth token." })),
            );
        }
    };

    let claims: Claims = match verify_jwt(&token) {
        Ok(claims) => claims,
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                response::Json(json!({ "error": "Invalid or expired token." })),
            );
        }
    };

    let user = match users.filter(user_id.eq(claims.user_id)).first::<User>(conn) {
        Ok(user) => user,
        Err(diesel::result::Error::NotFound) => {
            return (
                StatusCode::NOT_FOUND,
                response::Json(json!({ "error": "User not found." })),
            );
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                response::Json(json!({ "error": "Database query failed." })),
            );
        }
    };

    return (
        StatusCode::OK,
        response::Json(json!({
            "user": {
                "user_id": user.user_id,
                "email": user.email,
            }
        })),
    );
}
