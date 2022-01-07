use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CaptchaImage {
    pub captcha_on_off: bool,
    pub uuid: String,
    pub img: String,
}

#[derive(Debug, Serialize)]
/// 查 数据返回
pub struct ListData<T> {
    pub list: Vec<T>,
    pub total: usize,
    pub total_pages: usize,
    pub page_num: usize,
}

#[derive(Debug, Serialize)]
/// 增 删 改 数据返回
pub struct CudResData<T> {
    pub id: Option<T>,
    pub msg: String,
}
