use crate::{IdRef, IrItem};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use typescript_definitions::TypeScriptify;

#[derive(Debug, Default, Serialize, Deserialize, TypeScriptify)]
pub struct OutputRep {
    errors: HashMap<String, Vec<ErrorRep>>,
    refs: HashMap<String, Vec<String>>,
    items: HashMap<String, IrItem>,
}

#[derive(thiserror::Error, Debug, Serialize, Deserialize, TypeScriptify)]
#[serde(tag = "error_kind")]
pub enum ErrorRep {
    #[error("{given:?} not found")]
    NotFound { given: Option<String> },
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_deserialize_output_rep() -> eyre::Result<()> {
        let input1 = include_str!("../fixtures/output_rep.yaml");
        let output_rep: OutputRep = serde_yaml::from_str(input1)?;
        println!("{:?}", output_rep);
        let as_json = serde_json::to_string_pretty(&output_rep)?;
        println!("{}", as_json);
        Ok(())
    }
}
