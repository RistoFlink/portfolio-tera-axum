use std::collections::HashMap;

mod models;
mod routes;


use models::{AppState, Project};
use routes::create_router;

#[tokio::main]
async fn main() {
    let projects = load_projects();

    let tera = tera::Tera::new("templates/**/*")
        .expect("failed to compile tera");
    let app_state = AppState {
        tera,
        projects: std::sync::Arc::new(projects),
    };

    let app = create_router(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:7878").await.unwrap();
    println!("Server listening on port 7878");
    axum::serve(listener, app).await.unwrap();
}

// TODO: update this dummy function for data loading
fn load_projects() -> HashMap<String, Project> {
    HashMap::new()
}
