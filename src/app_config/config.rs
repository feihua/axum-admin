use serde::Deserialize;

// 定义配置结构体，包含服务器和数据库配置
#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub db: DbConfig,
    pub redis: RedisConfig,
    pub jwt: JwtConfig,
    pub log: LogConfig,
}

// 定义服务器配置结构体
#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub addr: String,
}

// 定义数据库配置结构体
#[derive(Debug, Deserialize)]
pub struct DbConfig {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct RedisConfig {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct JwtConfig {
    pub secret_key: String,
}

#[derive(Debug, Deserialize)]
pub struct LogConfig {
    pub directory: String,        //日志目录
    pub file_name_prefix: String, //日志文件名前缀
}
