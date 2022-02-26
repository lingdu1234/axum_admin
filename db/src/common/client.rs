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
