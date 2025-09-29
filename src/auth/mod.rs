pub mod model;
pub mod repository;
pub mod service;
pub mod controller;

use axum::Router;
use crate::auth::service::AuthService;

pub fn routes(auth_service: AuthService) -> Router {
    Router::new()
        .nest("/oauth", controller::routes(auth_service))
}
