pub mod controller;
pub mod model;
pub mod repository;
pub mod service;

use axum::{routing::{get, post}, Router};
use crate::core::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/verify", post(controller::initiate_verification))
        .route("/verify/status/:id", get(controller::get_verification_status))
        .route("/verify/complete", post(controller::complete_verification))
}