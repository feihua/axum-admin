use redis::Connection;

pub async fn init_redis() -> Connection {

    let client =redis::Client::open("redis://:r-wz9wop62956dh5k9ed@139.159.180.129:6379").unwrap();
    client.get_connection().unwrap()
}