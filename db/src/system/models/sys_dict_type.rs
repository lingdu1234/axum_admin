use serde::Deserialize;
#[derive(Deserialize, Debug)]
pub struct SearchReq {
    pub dict_type_id: Option<String>,
    pub dict_name: Option<String>,
    pub dict_type: Option<String>,
    pub status: Option<String>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct AddReq {
    pub dict_name: String,
    pub dict_type: String,
    pub status: String,
    pub remark: Option<String>,
}

#[derive(Deserialize)]
pub struct DeleteReq {
    pub dict_type_ids: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct EditReq {
    pub dict_type_id: String,
    pub dict_name: String,
    pub dict_type: String,
    pub status: String,
    pub remark: Option<String>,
}
