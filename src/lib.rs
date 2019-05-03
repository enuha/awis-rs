extern crate reqwest;
extern crate sha2;
extern crate hmac;
extern crate serde;
extern crate serde_xml_rs;

mod request;
mod serde_utils;
pub mod groups;
pub mod url_info;

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

    // Create a UrlInfo request for a url
    pub fn url_info(&self, response_group: &str, url: &str) -> Result<reqwest::Request, Box<error::Error>> {
        AwisClient::validate_groups(&url_info::VALID_GROUPS, response_group)?;

        let query = format!("Action={}&ResponseGroup={}&Url={}",
                            url_info::ACTION, response_group, url);
        request::new(&self.access_key, &self.secret_access_key, &query)
    }

    // Create a traffic history  request for a url
    pub fn traffic_history(&self, range: Option<i8>, start: Option<&str>, url: &str) -> Result<reqwest::Request, Box<error::Error>> {
        let query = match start {
            Some(st) => format!("Action={}&Range={}&ResponseGroup={}&Start={}&Url={}",
                                groups::traffic_history::ACTION, range.unwrap_or(31), groups::traffic_history::HISTORY, st, url),
            None => format!("Action={}&Range={}&ResponseGroup={}&Url={}",
                            groups::traffic_history::ACTION, range.unwrap_or(31), groups::traffic_history::HISTORY, url),
        };
        request::new(&self.access_key, &self.secret_access_key, &query)
    }

    pub fn category_browse(&self, descriptions: Option<bool>, path: &str, response_group: &str) -> Result<reqwest::Request, Box<error::Error>> {
        AwisClient::validate_groups(&groups::category_browse::VALID_GROUPS, response_group)?;

        let query = match descriptions {
            Some(st) => format!("Action={}&Descriptions={}&Path={}&ResponseGroup={}",
                                groups::category_browse::ACTION, st, path, response_group),
            None => format!("Action={}&Path={}&ResponseGroup={}", groups::category_browse::ACTION, path, response_group),
        };

        request::new(&self.access_key, &self.secret_access_key, &query)
    }

    pub fn category_listings(&self, count: Option<i8>, descriptions: Option<bool>, path: &str,
                             recursive: Option<bool>, sort_by: Option<&str>, start: Option<i32>) -> Result<reqwest::Request, Box<error::Error>> {
        let mut query: Vec<String> = Vec::with_capacity(8);

        query.push(format!("Action={}", groups::category_listing::ACTION));
        if let Some(c) = count { query.push(format!("Count={}", c)); }
        if let Some(d) = descriptions { query.push(format!("Descriptions={}", d)); }
        query.push(format!("Path={}", path));
        if let Some(r) = recursive { query.push(format!("Recursive={}", r)); }
        query.push(format!("ResponseGroup={}", groups::category_listing::LISTINGS));
        if let Some(s) = sort_by { query.push(format!("SortBy={}", s)); }
        if let Some(s) = start { query.push(format!("Start={}", s)); }

        request::new(&self.access_key, &self.secret_access_key, &query.join("&"))
    }

    pub fn sites_linking_in(&self, count: Option<i8>, start: Option<i32>, url: &str) -> Result<reqwest::Request, Box<error::Error>> {
        let mut query: Vec<String> = Vec::with_capacity(6);

        query.push(format!("Action={}", groups::sites_linking_in::ACTION));
        if let Some(c) = count { query.push(format!("Count={}", c)); }
        query.push(format!("ResponseGroup={}", groups::sites_linking_in::SITES_LINKING_IN));
        if let Some(s) = start { query.push(format!("Start={}", s)); }
        query.push(format!("Url={}", url));

        request::new(&self.access_key, &self.secret_access_key, &query.join("&"))
    }

    fn validate_groups(valid_groups: &[&str], response_group: &str) -> Result<(), Box<error::Error>> {
        for group in response_group.split(",") {
            if !valid_groups.contains(&group) {
                return err!("invalid response group {}", group);
            }
        };
        Ok(())
    }
}
