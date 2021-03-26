use ureq::{Request};

pub struct Client {
    pub api_key: String,
    pub sent_with: String,
}

impl Client {
    pub fn request(&self, method: &str, endpoint: &str) -> Request {
        ureq::request(method, &*format!("https://gateway.sms77.io/api/{}", endpoint))
            .set("X-API-KEY", &*self.api_key)
            .set("SentWith", &*self.sent_with)
    }

    pub fn bool_to_string(&self, value: bool) -> &str {
        if value {
            return "1";
        }

        "0"
    }

    pub fn new(api_key: String, sent_with: String) -> Self {
        Self {
            api_key,
            sent_with,
        }
    }
}