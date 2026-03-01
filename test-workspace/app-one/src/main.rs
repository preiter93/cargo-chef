use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    name: String,
    value: i32,
}

fn main() {
    let config = Config {
        name: "app-one".to_string(),
        value: 42,
    };

    let shared_value = lib_shared::get_shared_value();

    println!("Config: {:?}", config);
    println!("Shared value: {}", shared_value);
}
