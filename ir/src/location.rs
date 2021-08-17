use serde::{Deserialize, Serialize};
use typescript_definitions::TypeScriptify;

#[derive(Debug, Clone, Default, Deserialize, Serialize, TypeScriptify)]
pub struct Location {
    line_start: usize,
    line_end: usize,
}
