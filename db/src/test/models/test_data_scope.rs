use serde::Deserialize;
#[derive(Deserialize, Debug)]
pub struct SearchReq {
    pub data_a: Option<String>,
    pub data_b: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct AddReq {
    pub data_a: String,
    pub data_b: String,
}

#[derive(Deserialize)]
pub struct DeleteReq {
    pub ids: Vec<String>,
}
