use crate::{InputLanguage, IrItem};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use typescript_definitions::TypeScriptify;

#[derive(Debug, Default, Deserialize, Serialize, TypeScriptify)]
pub struct Config {
    name: String,
    content: Vec<IrItem>,
}

#[derive(Debug, Default, Deserialize, Serialize, TypeScriptify)]
pub struct ConfigDefinition {
    params: ConfigParams,
    config: String,
}

#[derive(Debug, Default, Deserialize, Serialize, TypeScriptify)]
pub struct ConfigParams {
    language: Option<InputLanguage>,
}