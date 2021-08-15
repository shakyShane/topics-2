use serde::{Deserialize, Deserializer, Serialize, Serializer};
use typescript_definitions::TypeScriptify;

#[derive(Debug, Deserialize, Serialize, TypeScriptify)]
pub enum InputLanguage {
    #[serde(rename = "toml")]
    Toml,
    #[serde(rename = "yaml")]
    Yaml,
    #[serde(rename = "shell")]
    Shell,
}
