use serde::{Deserialize, Serialize};
use typescript_definitions::TypeScriptify;

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq, Hash, TypeScriptify)]
pub struct Location {
    line_start: usize,
    line_end: usize,
}
