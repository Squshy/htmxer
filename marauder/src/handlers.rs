use crate::templates::{hehe, home as home_template};
use maud::Markup;

pub async fn home() -> Markup {
    home_template("hey", hehe())
}
