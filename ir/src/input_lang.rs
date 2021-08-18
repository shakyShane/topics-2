use serde::{Deserialize, Deserializer, Serialize, Serializer};
use typescript_definitions::TypeScriptify;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, TypeScriptify)]
pub enum InputLanguage {
    #[serde(rename = "toml")]
    Toml,
    #[serde(rename = "yaml")]
    Yaml,
    #[serde(rename = "shell")]
    Shell,
}
