use crate::location::Location;
use serde::{Deserialize, Serialize};

use std::hash::{Hash, Hasher};
use typescript_definitions::TypeScriptify;

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq, Hash, TypeScriptify)]
pub struct Markdown {
    content: String,
    loc: Option<Location>,
}

#[test]
fn test_hash() {
    use std::collections::hash_map::DefaultHasher;
    let mut hasher = DefaultHasher::new();
    let md = Markdown {
        content: String::from("# hey"),
        ..Default::default()
    };
    md.hash(&mut hasher);
    println!("{:?}", hasher.finish());
}
