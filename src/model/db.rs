use rbatis::rbatis::RBatis;

pub async fn init_db() -> RBatis {
    let rb = RBatis::new();
    rb.init(rbdc_mysql::driver::MysqlDriver {}, "mysql://root:oMbPi5munxCsBSsiLoPV@110.41.179.89:3306/salvodb").unwrap();
    return rb;
}