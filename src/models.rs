use serde::{ Serialize};
use std::{collections::HashMap, sync::Arc};
use tera::Tera;

#[derive(Clone)]
pub struct AppState {
    pub tera: Tera,
    pub projects: Arc<HashMap<String, Project>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Project {
    pub slug: String,
    pub name: String,
    pub hero: String,
    pub thumb: String,
    pub prod: String,
    pub categories: Vec<String>,
}