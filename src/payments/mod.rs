pub mod controller;
pub mod model;
pub mod repository;
pub mod service;

use axum::{routing::{get, post}, Router};
use crate::core::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(controller::create_payment))
        .route("/", get(controller::get_payments))
        .route("/:id", get(controller::get_payment_by_id))
        .route("/:id/cancel", post(controller::cancel_payment))
}