pub mod controller;
pub mod middleware;
pub mod model;
pub mod repository;
pub mod scopes;
pub mod service;

use crate::auth::service::AuthService;
use axum::Router;

pub fn routes(auth_service: AuthService) -> Router {
    Router::new().nest("/auth", controller::routes(auth_service))
}
