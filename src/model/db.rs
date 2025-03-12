use rbatis::rbatis::RBatis;

/// 初始化数据库连接
///
/// # Parameters
///
/// * `url`: &str - 数据库的连接URL
///
/// # Returns
///
/// * `RBatis` - 初始化后的RBatis实例
///
/// # Panics
///
/// 当数据库连接初始化失败时，该函数会通过`expect`方法抛出panic
pub async fn init_db(url: &str) -> RBatis {
    // 创建一个新的RBatis实例
    let rb = RBatis::new();
    // 使用提供的URL和MysqlDriver初始化RBatis实例
    rb.init(rbdc_mysql::driver::MysqlDriver {}, url)
        .expect("init db error");
    // 返回初始化后的RBatis实例
    rb
}
