use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct UserAddReq {
    pub user_name: String,
    pub mobile: String,
    // pub user_nickname: String,
    // pub birthday: i32,
    // pub user_password: String,
    // pub user_status: i8,
    pub user_email: String,
    // pub sex: i8,
    // pub avatar: String,
    // pub dept_id: String,
    // pub remark: String,
    // pub is_admin: i8,
    // pub address: String,
    // pub describe: String,
    // pub phone_num: String,
}
#[derive(PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
pub struct UserResp {
    pub user_name: String,
    pub mobile: String,
    pub user_nickname: String,
    pub birthday: i32,
    pub user_status: i8,
    pub user_email: String,
    pub sex: i8,
    pub avatar: String,
    pub dept_id: String,
    pub remark: String,
    pub is_admin: i8,
    pub address: String,
    pub describe: String,
    pub phone_num: String,
}

#[derive(Deserialize, Debug, Serialize, Default)]
pub struct UserSearchReq {
    pub user_id: Option<String>,
    pub user_name: Option<String>,
    pub user_nickname: Option<String>,
    pub user_status: Option<i8>,
    pub user_email: Option<String>,
    pub sex: Option<i8>,
    pub dept_id: Option<String>,
    pub phone_num: Option<String>,
}
