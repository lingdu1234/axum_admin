use serde::{Deserialize, Serialize};

pub mod sys_dict_type;
pub mod sys_user;

#[derive(Deserialize, Debug, Serialize, Default)]
pub struct PageParams {
    pub page_num: Option<usize>,
    pub page_size: Option<usize>,
}
