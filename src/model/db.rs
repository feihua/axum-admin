use rbatis::rbatis::RBatis;

pub async fn init_db() -> RBatis {
    let rb = RBatis::new();
    rb.init(
        rbdc_mysql::driver::MysqlDriver {},
        "mysql://root:oMbPi5munxCsBSsiLoPV1@110.41.179.89:3306/axum",
    )
    .unwrap();
    return rb;
}
