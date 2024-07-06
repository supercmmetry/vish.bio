use askama::Template;

#[derive(Template)]
#[template(path = "pages/index.html")]
pub struct IndexTemplate {
    name: String,
}

impl IndexTemplate {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}
