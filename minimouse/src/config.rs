use crate::templates::Template;
use axum::response::Html;
use minijinja::{path_loader, Environment, Value};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    PgPool,
};

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub tmpl_env: Environment<'static>,
}

impl AppState {
    pub fn new() -> Self {
        let mut env = Environment::new();
        env.set_loader(path_loader("templates"));
        Self {
            db_pool: get_connection_pool(),
            tmpl_env: env,
        }
    }

    pub fn render_template(&self, template: &Template, ctx: Value) -> Html<String> {
        let tmpl = self.tmpl_env.get_template(template.as_str()).unwrap();
        Html(tmpl.render(ctx).unwrap())
    }
}

fn get_connection_pool() -> PgPool {
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
