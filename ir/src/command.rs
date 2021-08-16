use serde::{Deserialize, Deserializer, Serialize, Serializer};
use typescript_definitions::TypeScriptify;

use crate::input_lang::InputLanguage;
use crate::IrItem;

#[derive(Debug, Default, Deserialize, Serialize, TypeScriptify)]
pub struct Command {
    pub(crate) name: String,
    content: Vec<IrItem>,
}

#[derive(Debug, Default, Deserialize, Serialize, TypeScriptify)]
pub struct CommandDefinition {
    command: String,
    params: CommandParams,
}

#[derive(Debug, Default, Deserialize, Serialize, TypeScriptify)]
pub struct CommandParams {
    language: Option<InputLanguage>,
}

#[derive(Debug, Default, Deserialize, Serialize, TypeScriptify)]
pub struct CommandConfig {
    config: String,
    params: CommandParams,
}

// impl Serialize for InputLanguage {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         serializer.serialize_str(match *self {
//             InputLanguage::Toml => "toml",
//             InputLanguage::Yaml => "yaml",
//             InputLanguage::Shell => "shell",
//         })
//     }
// }
//
// impl<'de> Deserialize<'de> for InputLanguage {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let s = String::deserialize(deserializer)?;
//         match s.as_str() {
//             "toml" => Ok(InputLanguage::Toml),
//             "yaml" | "yml" => Ok(InputLanguage::Yaml),
//             "shell" | "sh" | "bash" => Ok(InputLanguage::Shell),
//             _ => Err(serde::de::Error::custom(format!(
//                 "unsupported language '{}'",
//                 s
//             ))),
//         }
//     }
// }
