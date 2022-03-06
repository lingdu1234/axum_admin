use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct AddEditReq {
    pub api_id: String,
    pub dbs: Vec<String>,
}

#[derive(Deserialize)]
pub struct SearchReq {
    pub api_id: String,
}
