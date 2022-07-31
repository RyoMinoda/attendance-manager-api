use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Responce<T: Serialize> {
    success: bool,
    message: String,
    data: T
}

impl<T: Serialize> Responce<T> {
    pub fn new(data: T) -> String {
        let target = Responce {
            success: true,
            message: "".to_string(),
            data,
        };
        let json = serde_json::to_string(&target);
        json.expect("couldn't serialize data object")
    }
}