use crate::location::Location;
use crate::IrItem;
use serde::{Deserialize, Serialize};
use typescript_definitions::TypeScriptify;

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq, Hash, TypeScriptify)]
pub struct NamedRefList {
    pub content: Vec<IrItem>,
    loc: Option<Location>,
}
