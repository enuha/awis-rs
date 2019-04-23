extern crate reqwest;
extern crate sha2;
extern crate hmac;

mod request;
mod groups;

use reqwest::Request;
use reqwest::Client;
use std::error;

pub struct AwisClient {
    access_key: String,
    secret_access_key: String,
}


impl AwisClient {
    // create a new client
    pub fn new(access_key: &str, secret_access_key: &str) -> AwisClient {
        AwisClient {
            access_key: String::from(access_key),
            secret_access_key: String::from(secret_access_key),
        }
    }

    // Create a UrlInfo request for a domain
    pub fn url_info(&self, domain: &str, response_group: &str) -> Result<reqwest::Request, Box<error::Error>> {
        if !groups::urlinfo::VALID_GROUPS.contains(&response_group) {
            return err!("invalid response group {}", response_group);
        }
        let query = format!("Action=UrlInfo&ResponseGroup={}&Url={}", response_group, domain);
        request::new(&self.access_key, &self.secret_access_key, &query)
    }
}
