use crate::location::Location;
use crate::IrItem;
use serde::{Deserialize, Serialize};
use typescript_definitions::TypeScriptify;

#[derive(Debug, Default, Deserialize, Serialize, TypeScriptify)]
pub struct DependencyList {
    content: Vec<IrItem>,
    loc: Option<Location>,
}
