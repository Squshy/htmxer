pub enum Template {
    Layout(LayoutTemplate),
}

impl Template {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Layout(t) => t.as_str(),
        }
    }
}

pub enum LayoutTemplate {
    Home,
}

impl LayoutTemplate {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Home => "index.html",
        }
    }
}
