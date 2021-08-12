use crate::location::Location;

#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Markdown {
    content: String,
    loc: Option<Location>,
}
