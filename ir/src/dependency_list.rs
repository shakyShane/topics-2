use crate::location::Location;
use crate::IrItem;

#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct DependencyList {
    content: Vec<IrItem>,
    loc: Option<Location>,
}
