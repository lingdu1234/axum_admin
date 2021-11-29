use serde::Deserialize;

/// 配置文件
#[derive(Debug, Deserialize)]
pub struct Configs {
    /// 程序配置
    pub server: Server,
    /// Casbin配置
    pub casbin: Casbin,
    ///  数据库 配置
    pub database: Database,
    ///  JWT 配置
    pub jwt: Jwt,
    /// 日志配置
    pub log: Log,
}

/// server 配置文件
#[derive(Debug, Deserialize)]
pub struct Server {
    /// 服务器名称
    pub name: String,
    /// 服务器(IP地址:端口)     
    /// `0.0.0.0:3000`
    pub address: String,
}

/// casbin 配置文件
#[derive(Debug, Deserialize)]
pub struct Casbin {
    /// modelFile
    pub model_file: String,
    /// policyFile
    pub policy_file: String,
}

/// jwt 配置文件
#[derive(Debug, Deserialize)]
pub struct Jwt {
    /// JWT 密钥
    pub jwt_secret: String,
}

/// 日志配置
#[derive(Debug, Deserialize)]
pub struct Log {
    /// `log_level` 日志输出文件夹
    pub log_level: String,
    /// `dir` 日志输出文件夹
    pub dir: String,
    /// `file` 日志输出文件名
    pub file: String,
}

#[derive(Debug, Deserialize)]
pub enum DbType {
    MYSQL(String),
    POSTGRESQL,
    SQLITE,
}

/// 数据库
#[derive(Debug, Deserialize)]
pub struct Database {
    /// 数据库类型
    pub db_type: String,
    /// 数据库连接
    pub link: String,
}
