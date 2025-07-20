use crate::models::{AppState, Project};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use tera::{Context, Tera};
use tower_http::services::ServeDir;

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(home_handler))
        .route("/about", get(about_handler))
        .route("/contact", get(contact_handler))
        .route("/project/:slug", get(project_handler))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(app_state)
        .fallback(handler_404)
}

async fn home_handler() -> impl IntoResponse {}

async fn about_handler() -> impl IntoResponse {}

async fn contact_handler() -> impl IntoResponse {}

async fn project_handler() -> impl IntoResponse {}

async fn handler_404() -> impl IntoResponse {}