use reqwest::Client;

use super::ApiRequest;

const HOSTNAME: &str = "https://api.steampowered.com";

pub struct WebApiTransport {
    client: Client,
}

impl WebApiTransport {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    fn get_url(pathname: &str) -> String {
        format!("{HOSTNAME}/{pathname}")
    }

    async fn send(&self, request: ApiRequest) {
        let url = Self::get_url(&request.pathname());
        
    }
}

fn is_get_request(endpoint: &str) -> bool {
    endpoint == "IAuthenticationService/GetPasswordRSAPublicKey/v1"
}
