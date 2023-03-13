use rbatis::rbatis::Rbatis;

pub async fn init_db() -> Rbatis {
    let rb = Rbatis::new();
    rb.init(rbdc_mysql::driver::MysqlDriver {}, "mysql://root:r-wz9wop62956dh5k9ed@rm-wz9a2yv489d123yqkdo.mysql.rds.aliyuncs.com:3306/rustdb").unwrap();
    return rb;
}