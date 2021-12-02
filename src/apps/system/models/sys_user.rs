use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct UserAddReq {
    pub user_name: String,
    pub mobile: String,
    pub user_nickname: Option<String>,
    pub birthday: Option<i32>,
    pub user_password: String,
    pub user_status: Option<i8>,
    pub user_email: String,
    pub sex: Option<i8>,
    // pub avatar: Option<String>,
    pub dept_id: String,
    pub remark: Option<String>,
    pub is_admin: Option<i8>,
    pub address: Option<String>,
    pub describe: Option<String>,
    pub phone_num: Option<String>,
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
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

///  用户登录
#[derive(Deserialize, Debug, Serialize, Default)]
pub struct UserLoginReq {
    ///  用户名
    pub user_name: String,
    ///  用户密码
    pub user_password: String,
}
