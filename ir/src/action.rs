use crate::location::Location;
use crate::IrItem;

#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Action {
    name: String,
    content: Vec<IrItem>,
}
