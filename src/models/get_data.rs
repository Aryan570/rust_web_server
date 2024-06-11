use serde::Serialize;

#[derive(Serialize)]
pub struct GetData {
    pub key: String,
    pub value: String,
}
