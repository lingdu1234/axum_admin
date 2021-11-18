use once_cell::sync::Lazy;
use serde::Deserialize;
use std::{fs::File, io::Read};

const CFG_FILE: &str = "config/config.toml";
//  只要是配置文件中的配置项，都可以通过这个结构体来获取，只要读取一次值后保存到内存，一直可供使用
pub static CFG: Lazy<Configs> = Lazy::new(self::Configs::init);

impl Configs {
    pub fn init() -> Self {
        tracing::info!("配置文件开始初始化…………");
        let mut file = match File::open(CFG_FILE) {
            Ok(f) => f,
            Err(e) => panic!("不存在配置文件：{}，错误信息：{}", CFG_FILE, e),
        };
        let mut cfg_contents = String::new();
        match file.read_to_string(&mut cfg_contents) {
            Ok(s) => s,
            Err(e) => panic!("读取配置文件失败，错误信息：{}", e),
        };
        toml::from_str(&cfg_contents).expect("解析配置文件错误")
    }
}

#[derive(Debug, Deserialize)]
pub struct Configs {
    pub jwt: Jwt,
    pub log: Log,
}
#[derive(Debug, Deserialize)]
pub struct Jwt {
    pub jwt_secret: String,
}

#[derive(Debug, Deserialize)]
pub struct Log {
    pub dir: String,
    pub file: String,
}
