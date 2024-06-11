use serde::Deserialize;

#[derive(Deserialize,Debug)]
pub struct PostData {
    pub key: String,
    pub value: String,
}
