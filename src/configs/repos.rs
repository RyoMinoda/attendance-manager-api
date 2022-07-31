use dotenv::dotenv;
use std::env;

pub fn get_mock() -> bool {
    dotenv().ok();
    let mock = env::var("Mock").expect("Mock must be set");
    let b: bool = mock.parse().unwrap();
    b
}