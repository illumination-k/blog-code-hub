use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
struct User {
    #[schemars(title = "The user's name")]
    username: String,
}

fn main() {
    let schema = schemars::schema_for!(User);
    println!(
        "{}",
        serde_json::to_string_pretty(&schema)
        .unwrap()
    );
}
