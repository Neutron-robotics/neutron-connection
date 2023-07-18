extern crate redis;

pub fn get_connection(redis_connectionstring: &String) -> redis::Connection {
    redis::Client::open(redis_connectionstring.as_str())
        .expect("Invalid connection URL")
        .get_connection()
        .expect("failed to connect to Redis")
}
