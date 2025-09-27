pub mod controller;
pub mod model;
pub mod repository;
pub mod service;

use axum::{routing::{get, post}, Router};
use crate::core::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(controller::create_transaction))
        .route("/", get(controller::get_transactions))
        .route("/:id", get(controller::get_transaction_by_id))
        .route("/transfer", post(controller::transfer_funds))
}