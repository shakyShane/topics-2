use crate::location::Location;
use serde::{Deserialize, Serialize};
use typescript_definitions::TypeScriptify;

#[derive(Debug, Default, Deserialize, Serialize, TypeScriptify)]
pub struct NamedRef {
    name: String,
    loc: Option<Location>,
}
