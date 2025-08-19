use redis::Client;

pub async fn init_redis(url: &str) -> Client {
    redis::Client::open(url).expect("Invalid redis URL")
}
