pub mod controller;
pub mod model;
pub mod repository;
pub mod service;

use axum::{routing::{get, post}, Router};
use crate::core::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/verify", post(controller::initiate_income_verification))
        .route("/verify/status/:id", get(controller::get_income_verification_status))
        .route("/report", get(controller::get_income_report))
}