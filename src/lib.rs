mod operations;

pub use operations::*;

use vila::Client;

pub fn client(token: &str) -> Client {
    Client::new("https://api.openai.com").bearer_auth(token)
}
