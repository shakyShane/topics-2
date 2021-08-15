
use crate::IrItem;

#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Step {
    name: String,
    content: Vec<IrItem>,
}
