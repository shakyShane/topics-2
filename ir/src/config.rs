use crate::{InputLanguage, IrItem};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use typescript_definitions::TypeScriptify;

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq, Hash, TypeScriptify)]
pub struct Config {
    pub(crate) name: String,
    pub(crate) content: Vec<IrItem>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq, Hash, TypeScriptify)]
pub struct ConfigDefinition {
    params: ConfigParams,
    config: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq, Hash, TypeScriptify)]
pub struct ConfigParams {
    language: Option<InputLanguage>,
}
