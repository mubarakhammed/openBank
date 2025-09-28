pub mod controller;
pub mod model;
pub mod repository;
pub mod service;

use crate::core::AppState;
use axum::{routing::get, Router};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/balance", get(controller::get_balance))
        .route("/balance/history", get(controller::get_balance_history))
        .route("/profile", get(controller::get_user_profile))
        .route("/accounts", get(controller::get_user_accounts))
}
