use rbatis::rbatis::RBatis;

pub async fn init_db() -> RBatis {
    let rb = RBatis::new();
    rb.init(rbdc_mysql::driver::MysqlDriver {}, "mysql://root:ad879037-c7a4-4063-9236-6bfc35d54b7d@139.159.180.129:3306/rustdb").unwrap();
    return rb;
}