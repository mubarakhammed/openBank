pub mod controller;
pub mod model;
pub mod repository;
pub mod service;

use axum::{routing::get, Router};
use crate::core::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(controller::get_balance))
        .route("/history", get(controller::get_balance_history))
}