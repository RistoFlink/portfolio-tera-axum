use crate::models::{AppState, Project};
use axum::{
    Router,
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
};
use tera::{Context, Tera};
use tower_http::services::ServeDir;

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(home_handler))
        .route("/about", get(about_handler))
        .route("/contact", get(contact_handler))
        .route("/project/:slug", get(project_handler))
        .nest_service("/static", ServeDir::new("static"))
        .fallback(handler_404)
}

async fn home_handler(State(state): State<AppState>) -> impl IntoResponse {
    let projects_vec: Vec<Project> = state.projects.values().cloned().collect();

    let mut context = Context::new();
    context.insert("projects", &projects_vec);
    context.insert("current_page", "home");

    match state.tera.render("home.html", &context) {
        Ok(html) => Ok(Html(html)),
        Err(err) => {
            println!("Error rendering home.html: {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn about_handler(State(state): State<AppState>) -> impl IntoResponse {
    let mut context = Context::new();
    context.insert("current_page", "about");

    match state.tera.render("about.html", &context) {
        Ok(html) => Ok(Html(html)),
        Err(err) => {
            println!("Error rendering about.html: {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn contact_handler(State(state): State<AppState>) -> impl IntoResponse {
    let mut context = Context::new();
    context.insert("current_page", "contact");

    match state.tera.render("contact.html", &context) {
        Ok(html) => Ok(Html(html)),
        Err(err) => {
            println!("Error rendering contact.html: {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn project_handler(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    if let Some(project) = state.projects.get(&slug) {
        let template_name = format!("project_{}.html", slug);
        let mut context = Context::new();
        context.insert("project", &project);
        context.insert("current_page", "home");

        match state.tera.render(&template_name, &context) {
            Ok(html) => Ok(Html(html)),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn handler_404(State(state): State<AppState>) -> impl IntoResponse {
    let context = Context::new();
    let html = state
        .tera
        .render("404.html", &context)
        .unwrap_or_else(|_| "404 - Page not found".to_string());

    (StatusCode::NOT_FOUND, Html(html))
}
