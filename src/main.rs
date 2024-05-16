// Repo name: 65dd034

mod database;
mod movie;

use std::sync::Arc;

use axum::{
    extract::Path, http::StatusCode, response::{IntoResponse, Response}, routing::{get, post}, Json, Router
};
use database::Database;
use movie::Movie;
use serde::Deserialize;
use tokio::sync::RwLock;
use shadow_clone::shadow_clone;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database = Arc::new(RwLock::new(Database::new()));

    let app = Router::new()
        .route("/movie", post({
            shadow_clone!(database);
            move |body| create_movie(body, database)
        }))
        .route("/movie/:id", get({
            shadow_clone!(database);
            move |path| get_movie(path, database)
        }));

    eprintln!("server listning on 127.0.0.1:3000");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn create_movie(Json(payload): Json<CreateMoviePayload>, database: Arc<RwLock<Database>>) -> Response {
    let movie = Movie {
        id: payload.id,
        name: payload.name,
        year: payload.year,
        was_good: payload.was_good,
    };

    let result = database.write().await.insert(movie.clone());

    match result {
        Ok(_) => (StatusCode::CREATED, Json(movie)).into_response(), // todo figure out how to return json and string
        Err(err) => (StatusCode::CONFLICT, err).into_response(),
    }
}

async fn get_movie(Path(movie_id): Path<String>, database: Arc<RwLock<Database>>) -> Response {
    let result = database.read().await.lookup_by_id(&movie_id).cloned();

    match result {
        Some(movie) => (StatusCode::OK, Json(movie)).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

#[derive(Deserialize)]
struct CreateMoviePayload {
    id: String,
    name: String,
    year: u16,
    was_good: bool
}