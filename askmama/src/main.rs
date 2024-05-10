mod templates;
mod todo;

use axum::{
    extract::State,
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Form, Router,
};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    PgPool,
};
use templates::{Home, HtmlTemplate};
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

async fn create_todo(State(state): State<AppState>, Form(todo): Form<TodoRequest>) -> Redirect {
    sqlx::query!(
        r#"
        INSERT INTO todos 
            (id, title, completed, created_at)
        VALUES 
            ($1, $2, $3, $4) 
        "#,
        uuid::Uuid::new_v4(),
        todo.title,
        todo.completed == Some("on".to_string()),
        chrono::Utc::now(),
    )
    .execute(&state.db_pool)
    .await
    .expect("Failed to insert todo");

    Redirect::to("/")
}

async fn home(State(state): State<AppState>) -> impl IntoResponse {
    let todos = sqlx::query_as!(Todo, r#"SELECT * FROM todos"#)
        .fetch_all(&state.db_pool)
        .await
        .expect("Failed to fetch todos");

    for todo in todos.iter() {
        println!("{}", todo.id);
    }

    HtmlTemplate(Home { todos })
}
