pub mod controller;
pub mod model;
pub mod repository;
pub mod service;

use axum::{routing::{get, post}, Router};
use crate::core::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(controller::create_virtual_account))
        .route("/", get(controller::get_virtual_accounts))
        .route("/:id", get(controller::get_virtual_account_by_id))
        .route("/:id/deactivate", post(controller::deactivate_virtual_account))
}