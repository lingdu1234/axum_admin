use serde::Deserialize;

/// 配置文件
#[derive(Debug, Deserialize)]
pub struct Configs {
    /// 程序配置
    pub server: Server,
    /// 静态网站配置
    pub web: Web,
    /// cert配置
    pub cert: Cert,
    /// 系统配置
    pub system: System,
    ///  数据库 配置
    pub database: Database,
    ///  JWT 配置
    pub jwt: Jwt,
    /// 日志配置
    pub log: Log,
    /// skytable
    pub skytable: SkyTable,
}

/// server 配置文件
#[derive(Debug, Deserialize)]
pub struct Server {
    /// 服务器名称
    pub name: String,
    /// 服务器(IP地址:端口)
    /// `0.0.0.0:3000`
    pub address: String,
    /// 服务器ssl
    pub ssl: bool,
    /// 响应数据gzip
    pub content_gzip: bool,
    /// 缓存时间
    pub cache_time: u64,
    /// 缓存方式
    pub cache_method: u32,
    /// api 前缀  例如："/api_v1"
    pub api_prefix: String,
}

/// server 配置文件
#[derive(Debug, Deserialize)]
pub struct Web {
    /// 静态网站根目录
    pub dir: String,
    /// 静态网站index文件名
    /// `index.html`
    pub index: String,
    /// 文件上传路径
    pub upload_dir: String,
    /// 文件上传路径
    pub upload_url: String,
}
#[derive(Debug, Deserialize)]
pub struct Cert {
    /// cert
    pub cert: String,

    /// key
    pub key: String,
}
/// system 系统配置
#[derive(Debug, Deserialize)]
pub struct System {
    /// 超级管理员账号
    pub super_user: Vec<String>,
    /// user agent 解析
    pub user_agent_parser: String,
}

/// jwt 配置文件
#[derive(Debug, Deserialize)]
pub struct Jwt {
    /// JWT 密钥
    pub jwt_secret: String,
    /// JWT 过期时间
    pub jwt_exp: i64,
}

/// 日志配置
#[derive(Debug, Deserialize)]
pub struct Log {
    /// `log_level` 日志输出等级
    pub log_level: String,
    /// `dir` 日志输出文件夹
    pub dir: String,
    /// `file` 日志输出文件名
    pub file: String,
    /// 允许操作日志输出
    pub enable_oper_log: bool,
}

/// 数据库
#[derive(Debug, Deserialize)]
pub struct Database {
    /// 数据库连接
    pub link: String,
}

#[derive(Debug, Deserialize)]
pub struct SkyTable {
    /// server address
    pub server: String,

    /// server port
    pub port: u16,
}
