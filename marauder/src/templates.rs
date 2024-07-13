use maud::{html, Markup, DOCTYPE};

pub fn hehe() -> Markup {
    html! {
        h1 { "idk" }
    }
}

pub fn home(title: &'static str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                title { (title) }
                link rel="stylesheet" href="/assets/main.css";
                script
                    src="https://unpkg.com/htmx.org@1.9.2"
                    integrity="sha384-L6OqL9pRWyyFU3+/bjdSri+iIphTN/bvYyM37tICVyOJkWZLpP2vGn6VUEXgzg6h"
                    crossorigin="anonymous";
            }
            body class="bg-slate-900 text-white" {
                (content)
            }
        }
    }
}
