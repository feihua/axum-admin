use crate::app_config::config::AppConfig;
use config::{Config, File};

/// 加载和解析配置文件
pub fn init_config(file_name: &str) -> AppConfig {
    let config = Config::builder().add_source(File::with_name(file_name)).build().unwrap().try_deserialize::<AppConfig>().unwrap();
    println!("load config: {:?}", config);
    config
}
