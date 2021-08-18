use crate::location::Location;
use crate::IrItem;
use serde::{Deserialize, Serialize};
use typescript_definitions::TypeScriptify;

#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq, Hash, Serialize, TypeScriptify)]
pub struct DependencyList {
    pub content: Vec<IrItem>,
    loc: Option<Location>,
}
