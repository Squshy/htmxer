use crate::{
    templates::{LayoutTemplate, Template},
    AppState,
};
use axum::{extract::State, response::IntoResponse};
use minijinja::context;

pub async fn home(State(state): State<AppState>) -> impl IntoResponse {
    state.render_template(
        &Template::Layout(LayoutTemplate::Home),
        context! {
            haha => "hehe"
        },
    )
}
