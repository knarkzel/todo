use axum::{
    extract::{Path, State},
    routing::get,
    Form, Json, Router,
};
use axum_error::Result;
use serde::{Deserialize, Serialize};
use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePool, Sqlite};
use std::net::SocketAddr;
use tracing::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Logging and environment variables
    let _ = dotenv::dotenv();
    tracing_subscriber::fmt::init();

    // Create database and run migrations
    let url = std::env::var("DATABASE_URL")?;
    if !Sqlite::database_exists(&url).await? {
        Sqlite::create_database(&url).await?;
    }

    // Create connection and share it using with_state below
    let pool = SqlitePool::connect(&url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    // Create router for server
    let app = Router::new()
        .route("/create", get(create))
        .route("/read/:id", get(read))
        .route("/update", get(update))
        .route("/delete/:id", get(delete))
        .route("/list", get(list))
        .with_state(pool);

    // Start server!
    let address = SocketAddr::from(([0, 0, 0, 0], 8000));
    info!("Starting server on http://{address}");
    Ok(axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await?)
}

#[derive(Deserialize)]
struct NewTodo {
    description: String,
    done: bool,
}

#[derive(Serialize, Deserialize)]
struct Todo {
    id: i64,
    description: String,
    done: bool,
}

async fn create(State(pool): State<SqlitePool>, Form(todo): Form<NewTodo>) -> Result<String> {
    // Create new note
    sqlx::query!(
        "INSERT INTO todos (description, done) VALUES (?, ?)",
        todo.description,
        todo.done
    )
    .execute(&pool)
    .await?;
    Ok(format!("Succesfully inserted todo!"))
}

async fn read(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Result<Json<Todo>> {
    // Read todo
    let todo = sqlx::query_as!(
        Todo,
        "SELECT id, description, done FROM todos WHERE id = ?",
        id
    )
    .fetch_one(&pool)
    .await?;
    Ok(Json(todo))
}

async fn update(State(pool): State<SqlitePool>, Form(todo): Form<Todo>) -> Result<String> {
    // Update todo
    sqlx::query!(
        "UPDATE todos SET description = ?, DONE = ? WHERE id = ?",
        todo.description,
        todo.done,
        todo.id
    )
    .execute(&pool)
    .await?;
    Ok(format!("Succesfully updated todo!"))
}

async fn delete(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Result<String> {
    // Update todo
    sqlx::query!("DELETE FROM todos WHERE id = ?", id)
        .execute(&pool)
        .await?;
    Ok(format!("Succesfully deleted todo!"))
}

async fn list(State(pool): State<SqlitePool>) -> Result<Json<Vec<Todo>>> {
    // List all notes
    let todos = sqlx::query_as!(Todo, "SELECT id, description, done FROM todos ORDER BY id")
        .fetch_all(&pool)
        .await?;
    Ok(Json(todos))
}
