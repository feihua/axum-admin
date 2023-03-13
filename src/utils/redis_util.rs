use redis::Connection;

pub async fn init_redis() -> Connection {

    let client =redis::Client::open("redis://:r-wz9wop62956dh5k9ed@r-wz9wop62956dh5k9edpd.redis.rds.aliyuncs.com:6379").unwrap();
    client.get_connection().unwrap()
}