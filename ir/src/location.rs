#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Location {
    line_start: usize,
    line_end: usize,
}
