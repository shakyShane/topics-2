use crate::location::Location;
use serde::{Deserialize, Serialize};
use typescript_definitions::TypeScriptify;

#[derive(Debug, Default, Deserialize, Serialize, TypeScriptify)]
pub struct Markdown {
    content: String,
    loc: Option<Location>,
}
