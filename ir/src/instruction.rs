use crate::location::Location;
use crate::IrItem;

#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Instruction {
    name: String,
    loc: Option<Location>,
    content: Vec<IrItem>,
}
