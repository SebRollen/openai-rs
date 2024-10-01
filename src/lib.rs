mod operations;

pub use operations::*;

use tower_api_client::Client;

pub fn client(token: &str) -> Client {
    Client::new("https://api.openai.com").bearer_auth(token)
}
