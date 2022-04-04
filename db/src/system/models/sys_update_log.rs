use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct AddReq {
    pub app_version: String,
    pub backend_version: String,
    pub title: String,
    pub content: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct EditReq {
    pub id: String,
    pub app_version: String,
    pub backend_version: String,
    pub title: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct DeleteReq {
    pub id: String,
}
