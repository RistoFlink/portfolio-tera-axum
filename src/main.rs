use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

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
        tera: Arc::new(tera),
        projects: Arc::new(projects),
    };

    let app = create_router();

    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("Failed to parse PORT");

    let address = SocketAddr::from(([0, 0, 0, 0], port));

    let app_with_state = app.with_state(app_state);

    let listener = tokio::net::TcpListener::bind(address).await.expect("Failed to bind to address");
    println!("Server listening on port {port}");
    axum::serve(listener, app_with_state).await.unwrap();
}

fn load_projects() -> HashMap<String, Project> {
    let projects_data = vec![
        Project {
            name: "Microservices with Go, Docker, gRPC, and RabbitMQ".to_string(),
            thumb: "img/habit-tracking.png".to_string(),
            hero: "img/microservices-go.png".to_string(),
            categories: vec!["go".to_string(), "rabbitmq".to_string(), "grpc".to_string()],
            slug: "microservices-go".to_string(),
            prod: "https://swarm.ristoflink.dev".to_string(),
        },
        Project {
            name: "Book recommendation with Python, LangChain, and Gradio".to_string(),
            thumb: "img/habit-tracking.png".to_string(),
            hero: "img/book-recommender.png".to_string(),
            categories: vec!["python".to_string(), "llm".to_string(), "openai".to_string()],
            slug: "book-recommender".to_string(),
            prod: "https://book-recommender-rf.up.railway.app".to_string(),
        },
        Project {
            name: "Habit tracking app with Python and MongoDB".to_string(),
            thumb: "img/habit-tracking.png".to_string(),
            hero: "img/habit-tracker-code.png".to_string(),
            categories: vec!["python".to_string(), "web".to_string(), "mongodb".to_string()],
            slug: "habit-tracking".to_string(),
            prod: "https://habit-tracker-flask.up.railway.app".to_string(),
        },
        Project {
            name: "Microblog with Python and MongoDB".to_string(),
            thumb: "img/habit-tracking.png".to_string(),
            hero: "img/microblog.png".to_string(),
            categories: vec!["python".to_string(), "web".to_string(), "mongodb".to_string()],
            slug: "microblog".to_string(),
            prod: "https://python-microblog-production.up.railway.app/".to_string(),
        },
        Project {
            name: "Task list with Go, React and MongoDB".to_string(),
            thumb: "img/habit-tracking.png".to_string(),
            hero: "img/tasklist.png".to_string(),
            categories: vec!["go".to_string(), "react".to_string(), "mongodb".to_string()],
            slug: "task-list".to_string(),
            prod: "https://go-tasks-production.up.railway.app/".to_string(),
        },
        Project {
            name: "Expanding an online store into the Estonian market".to_string(),
            thumb: "img/habit-tracking.png".to_string(),
            hero: "img/habit-tracking-hero.png".to_string(),
            categories: vec!["writing".to_string()],
            slug: "thesis".to_string(),
            prod: "https://urn.fi/URN:NBN:fi:amk-2020100721092".to_string(),
        },
    ];

    // Convert Vec<Project> to HashMap<String, Project> keyed by slug
    projects_data
        .into_iter()
        .map(|project| (project.slug.clone(), project))
        .collect()
}
