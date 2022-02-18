use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct ClientNetInfo {
    pub ip: String,
    pub location: String,
    pub net_work: String,
}

#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct UserAgentInfo {
    pub browser: String,
    pub os: String,
    pub device: String,
}

#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct ClientInfo {
    pub net: ClientNetInfo,
    pub ua: UserAgentInfo,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ReqInfo {
    pub path: String,
    pub ori_path: String,
    pub method: String,
    pub user: String,
    pub user_id: String,
    pub client_info: ClientInfo,
    pub data: String,
    pub query: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ResInfo {
    pub duration: String,
    pub status: String,
    pub data: String,
    pub err_msg: String,
}
