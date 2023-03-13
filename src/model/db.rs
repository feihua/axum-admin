use rbatis::rbatis::Rbatis;

pub async fn init_db() -> Rbatis {
    let rb = Rbatis::new();
    rb.init(rbdc_mysql::driver::MysqlDriver {}, "mysql://root:123456@127.0.0.1:3306/rustdb").unwrap();
    return rb;
}