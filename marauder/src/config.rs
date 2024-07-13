use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    PgPool,
};

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            db_pool: get_connection_pool(),
        }
    }
}

fn get_connection_pool() -> PgPool {
    let options = PgConnectOptions::new()
        .host("localhost")
        .username("postgres")
        .password("password")
        .database("askmama") // yeah good enuf
        .port(5432);

    PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(options)
}
