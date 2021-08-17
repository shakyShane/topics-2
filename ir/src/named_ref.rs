use crate::location::Location;
use serde::{Deserialize, Serialize};
use typescript_definitions::TypeScriptify;

#[derive(Debug, Clone, Default, Deserialize, Serialize, TypeScriptify)]
pub struct NamedRef {
    name: String,
    loc: Option<Location>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, TypeScriptify)]
pub struct IdRef {
    pub(crate) id: String,
}
