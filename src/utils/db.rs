use rbatis::rbatis::RBatis;
use rbatis::rbdc::pool::{ConnectionManager, Pool};
use rbdc_mysql::MysqlDriver;
use rbdc_pool_fast::FastPool;

pub async fn init_db(url: &str) -> RBatis {
    let rb = RBatis::new();

    let manager = ConnectionManager::new(MysqlDriver {}, url).expect("create connection manager error");
    let pool = FastPool::new(manager).expect("create db pool error");

    rb.init_pool(pool).expect("init db pool error");
    rb
}
