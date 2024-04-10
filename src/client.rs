use ureq::{Request};

pub struct Client {
    pub api_key: String,
    pub sent_with: String,
}

impl Client {

    pub fn delete(&self, endpoint: &str) -> Request {
        self.request("DELETE", endpoint)
    }
    pub fn get(&self, endpoint: &str) -> Request {
        self.request("GET", endpoint)
    }

    pub fn post(&self, endpoint: &str) -> Request {
        self.request("POST", endpoint)
    }

    pub fn patch(&self, endpoint: &str) -> Request {
         self.request("PATCH", endpoint)
    }

    pub fn request(&self, method: &str, endpoint: &str) -> Request {
        ureq::request(method, &*format!("https://gateway.seven.io/api/{}", endpoint))
            .set("Accept", "application/json")
            .set("SentWith", &*self.sent_with)
            .set("X-Api-Key", &*self.api_key)
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
