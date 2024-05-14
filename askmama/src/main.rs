mod templates;
mod todo;

use askama::Template;
use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
    routing::{delete, get, post},
    Form, Router,
};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    PgPool,
};
use templates::{Home, HtmlTemplate, TodoNotFound, TodoRow, TodoRows, TodoScene};
use todo::Todo;
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[derive(Clone)]
struct AppState {
    db_pool: PgPool,
}

pub fn get_connection_pool() -> PgPool {
    let options = PgConnectOptions::new()
        .host("localhost")
        .username("postgres")
        .password("password")
        .database("askmama")
        .port(5432);

    PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(options)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("initializing app...");

    let assets_path = std::env::current_dir().unwrap();
    let app_state = AppState {
        db_pool: get_connection_pool(),
    };
    let app = Router::new()
        .route("/", get(home))
        .route("/todos", post(create_todo))
        .route("/todos", get(list_todos))
        .route("/todos/:id", delete(delete_todo))
        .route("/todos/:id", get(show_todo))
        .route("/todos/:id", post(update_todo))
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
        )
        .with_state(app_state);

    let port = 42069;
    info!("app initialized, now listening on port {}", port);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(serde::Deserialize)]
struct TodoRequest {
    title: String,
    completed: Option<String>,
}

async fn create_todo(
    State(state): State<AppState>,
    Form(todo): Form<TodoRequest>,
) -> impl IntoResponse {
    let todo = sqlx::query_as!(
        Todo,
        r#"
        INSERT INTO todos 
            (id, title, completed, created_at)
        VALUES 
            ($1, $2, $3, $4) 
        RETURNING *
        "#,
        uuid::Uuid::new_v4(),
        todo.title,
        todo.completed == Some("on".to_string()),
        chrono::Utc::now(),
    )
    .fetch_one(&state.db_pool)
    .await
    .expect("Failed to insert todo");

    HtmlTemplate(TodoRow { todo })
}

async fn show_todo(State(state): State<AppState>, Path(id): Path<uuid::Uuid>) -> impl IntoResponse {
    let todo = sqlx::query_as!(Todo, r#"SELECT * FROM todos WHERE id = $1"#, id)
        .fetch_one(&state.db_pool)
        .await;

    // I have no idea how to pass back the `<head>` properly if a user re-freshes
    // the page since I cannot have nested imports/extends in these templates.
    // I _could_ check that it was an HTMX request with the `Hx-Request` header
    // however, how to send back the CSS and scripts then? Do I need different
    // templates then if I cannot nest this in a conditional?
    if todo.is_err() {
        return Html(TodoNotFound { id }.render().unwrap()).into_response();
    }

    return Html(
        TodoScene {
            todo: todo.unwrap(),
        }
        .render()
        .unwrap(),
    )
    .into_response();
}

#[derive(serde::Deserialize, Debug)]
struct TodoUpdateRequest {
    title: Option<String>,
    completed: Option<String>,
}

async fn update_todo(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
    Form(updates): Form<TodoUpdateRequest>,
) -> impl IntoResponse {
    println!("ID: {}", id);
    let mut tx = state.db_pool.begin().await.unwrap();
    // Lots of optimizations here ofc
    let todo = sqlx::query!(r#"SELECT title, completed FROM todos WHERE id = $1"#, id)
        .fetch_one(&mut tx)
        .await
        .expect("Did not find todo");

    let title = updates.title.unwrap_or(todo.title);
    let completed = updates.completed.map(|c| c == "on").unwrap_or(false);

    println!("New title: {}", title);
    println!("New completed: {}", completed);
    let todo = sqlx::query_as!(
        Todo,
        r#"UPDATE todos SET title = $1, completed = $2 WHERE id = $3 RETURNING *"#,
        title,
        completed,
        id
    )
    .fetch_one(&mut tx)
    .await
    .expect("failed to update todo");
    tx.commit().await.unwrap();

    println!("{}", todo.title);
    println!("{}", todo.completed);

    HtmlTemplate(TodoRow { todo })
}

async fn delete_todo(State(state): State<AppState>, Path(id): Path<uuid::Uuid>) {
    sqlx::query!(r#"DELETE FROM todos WHERE id = $1"#, id)
        .execute(&state.db_pool)
        .await
        .expect("Failed to delete todo");
}

async fn list_todos(State(state): State<AppState>) -> impl IntoResponse {
    let todos = sqlx::query_as!(Todo, r#"SELECT * FROM todos ORDER BY created_at DESC"#)
        .fetch_all(&state.db_pool)
        .await
        .expect("Failed to fetch todos");

    HtmlTemplate(TodoRows { todos })
}

async fn home() -> impl IntoResponse {
    HtmlTemplate(Home {})
}
