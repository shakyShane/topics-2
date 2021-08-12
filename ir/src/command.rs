use crate::location::Location;

#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Command {
    name: String,
    description: Option<String>,
    loc: Option<Location>,
    raw: String,
}
