use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

use crate::todo::Todo;

#[derive(Template)]
#[template(path = "home.html")]
pub struct Home {}

#[derive(Template)]
#[template(path = "todo_rows.html")]
pub struct TodoRows {
    pub todos: Vec<Todo>,
}

#[derive(Template)]
#[template(path = "todo_row.html")]
pub struct TodoRow {
    pub todo: Todo,
}

#[derive(Template)]
#[template(path = "todo_not_found.html")]
pub struct TodoNotFound {
    pub id: uuid::Uuid,
}

#[derive(Template)]
#[template(path = "todo_page.html")]
pub struct TodoScene {
    pub todo: Todo,
}

/// A wrapper type that we'll use to encapsulate HTML parsed by askama into valid HTML for axum to serve.
pub struct HtmlTemplate<T>(pub T);

/// Allows us to convert Askama HTML templates into valid HTML for axum to serve in the response.
impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        // Attempt to render the template with askama
        match self.0.render() {
            // If we're able to successfully parse and aggregate the template, serve it
            Ok(html) => Html(html).into_response(),
            // If we're not, return an error or some bit of fallback HTML
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}
