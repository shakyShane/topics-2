use crate::location::Location;

#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct NamedRef {
    name: String,
    loc: Option<Location>,
}
