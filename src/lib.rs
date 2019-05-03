extern crate hmac;
extern crate reqwest;
extern crate serde;
extern crate serde_xml_rs;
extern crate sha2;

pub mod groups;
mod request;
mod serde_utils;
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
    pub fn url_info(
        &self,
        response_group: &str,
        url: &str,
    ) -> Result<reqwest::Request, Box<error::Error>> {
        AwisClient::validate_groups(&url_info::VALID_GROUPS, response_group)?;

        request::new(
            &self.access_key,
            &self.secret_access_key,
            &[
                ("Action", url_info::ACTION),
                ("ResponseGroup", response_group),
                ("Url", url),
            ],
        )
    }

    // Create a traffic history  request for a url
    pub fn traffic_history(
        &self,
        range: Option<i8>,
        start: Option<&str>,
        url: &str,
    ) -> Result<reqwest::Request, Box<error::Error>> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        params.push(("Action", groups::traffic_history::ACTION));

        let mut long_living = String::new();
        if let Some(rg) = range {
            long_living.push_str(&rg.to_string());
            params.push(("Range", &long_living));
        }

        params.push(("ResponseGroup", groups::traffic_history::HISTORY));
        if let Some(st) = start {
            params.push(("Start", st));
        }
        params.push(("Url", url));
        request::new(&self.access_key, &self.secret_access_key, &params)
    }

    pub fn category_browse(
        &self,
        descriptions: Option<bool>,
        path: &str,
        response_group: &str,
    ) -> Result<reqwest::Request, Box<error::Error>> {
        AwisClient::validate_groups(&groups::category_browse::VALID_GROUPS, response_group)?;

        let mut params: Vec<(&str, &str)> = Vec::new();
        params.push(("Action", groups::category_browse::ACTION));

        let mut long_living = String::new();
        if let Some(ds) = descriptions {
            long_living.push_str(&ds.to_string());
            params.push(("Descriptions", &long_living));
        }

        params.push(("Path", path));
        params.push(("ResponseGroup", response_group));

        request::new(&self.access_key, &self.secret_access_key, &params)
    }

    pub fn category_listings(
        &self,
        count: Option<i8>,
        descriptions: Option<bool>,
        path: &str,
        recursive: Option<bool>,
        sort_by: Option<&str>,
        start: Option<i32>,
    ) -> Result<reqwest::Request, Box<error::Error>> {
        let mut params: Vec<(&str, &str)> = Vec::with_capacity(8);

        params.push(("Action", groups::category_listing::ACTION));
        let mut long_living = String::new();
        if let Some(c) = count {
            long_living.push_str(&c.to_string());
            params.push(("Count", &long_living));
        }

        let mut long_living = String::new();
        if let Some(d) = descriptions {
            long_living.push_str(&d.to_string());
            params.push(("Descriptions", &long_living));
        }

        params.push(("Path", path));

        let mut long_living = String::new();
        if let Some(r) = recursive {
            long_living.push_str(&r.to_string());
            params.push(("Recursive", &long_living));
        }

        params.push(("ResponseGroup", groups::category_listing::LISTINGS));

        if let Some(s) = sort_by {
            params.push(("SortBy", s));
        }

        let mut long_living = String::new();
        if let Some(s) = start {
            long_living.push_str(&s.to_string());
            params.push(("Start", &long_living));
        }

        request::new(&self.access_key, &self.secret_access_key, &params)
    }

    pub fn sites_linking_in(
        &self,
        count: Option<i8>,
        start: Option<i32>,
        url: &str,
    ) -> Result<reqwest::Request, Box<error::Error>> {
        let mut params: Vec<(&str, &str)> = Vec::with_capacity(6);

        params.push(("Action", groups::sites_linking_in::ACTION));

        let mut long_living = String::new();
        if let Some(c) = count {
            long_living.push_str(&c.to_string());
            params.push(("Count", &long_living));
        }

        params.push(("ResponseGroup", groups::sites_linking_in::SITES_LINKING_IN));

        let mut long_living = String::new();
        if let Some(s) = start {
            long_living.push_str(&s.to_string());
            params.push(("Start", &long_living));
        }
        params.push(("Url", url));

        request::new(&self.access_key, &self.secret_access_key, &params)
    }

    fn validate_groups(
        valid_groups: &[&str],
        response_group: &str,
    ) -> Result<(), Box<error::Error>> {
        for group in response_group.split(",") {
            if !valid_groups.contains(&group) {
                return err!("invalid response group {}", group);
            }
        }
        Ok(())
    }
}
