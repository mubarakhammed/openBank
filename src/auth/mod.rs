pub mod controller;
pub mod model;
pub mod repository;
pub mod service;

use axum::{routing::post, Router};
use crate::core::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(controller::register))
        .route("/login", post(controller::login))
        .route("/refresh", post(controller::refresh_token))
        .route("/logout", post(controller::logout))
}