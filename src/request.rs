use std::error;

use chrono::Utc;
use hmac::{Hmac, Mac};
use reqwest::{Method, Request, Url};
use reqwest::header::HeaderValue;
use sha2::{Digest, Sha256};

type Hmac256 = Hmac<Sha256>;

#[macro_export]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<error::Error>::from(format!($($tt)*))) }
}

const METHOD: &str = "GET";
const SERVICE: &str = "awis";
const HOST: &str = "awis.us-west-1.amazonaws.com";
const REGION: &str = "us-west-1";
const ENDPOINT: &str = "https://awis.amazonaws.com/api";
const CANONICAL_URI: &str = "/api";
const SIGNED_HEADERS: &str = "host;x-amz-date";
const ALGORITHM: &str = "AWS4-HMAC-SHA256";

// the payload is always empty -- this is the sha256 for empty string
const PAYLOAD_HASH: &str = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

// Signs a request
pub fn new(access_key_id: &str, secret_access_key: &str, canonical_query: &str) -> Result<Request, Box<error::Error>> {
    let amz_date: String = Utc::now().format("%Y%m%dT%H%M%SZ").to_string();
    let date_stamp: String = Utc::now().format("%Y%m%d").to_string();
    let credential_scope = format!("{}/{}/{}/aws4_request", date_stamp, REGION, SERVICE);


    let canonical_request_hash = request_hash(canonical_query, &amz_date);

    // the string we'll sign
    let string_to_sign = format!("{}\n{}\n{}\n{}", ALGORITHM, amz_date, credential_scope, canonical_request_hash);

    // the key we'll use to sign our request
    let signature = signature_key(secret_access_key, &date_stamp, REGION, SERVICE);

    let mut sg = Hmac256::new_varkey(&signature).unwrap();
    sg.input(string_to_sign.as_bytes());
    let signature = format!("{:x}", sg.result().code());

    let auth_header_value = format!("{} Credential={}/{}, SignedHeaders={}, Signature={}",
                                    ALGORITHM, access_key_id, credential_scope, SIGNED_HEADERS, signature);


    let url = Url::parse(&format!("{}?{}", ENDPOINT, canonical_query))?;
    let mut req = Request::new(Method::GET, url);
    let headers = req.headers_mut();


    headers.insert("x-amz-date", HeaderValue::from_str(&amz_date)?);
    headers.insert("Authorization", HeaderValue::from_str(&auth_header_value)?);
    Ok(req)
}


// create the hash of the canonical request
fn request_hash(canonical_query: &str, amz_date: &str) -> String {
    let canonical_headers: &str = &format!("host:{}\nx-amz-date:{}", HOST, amz_date);
    let canonical_request: &str = &format!("{}\n{}\n{}\n{}\n\n{}\n{}",
                                           METHOD, CANONICAL_URI, canonical_query, canonical_headers,
                                           SIGNED_HEADERS, PAYLOAD_HASH);

    format!("{:x}", sha2::Sha256::digest(canonical_request.as_bytes()))
}

// generate the signature key
fn signature_key(key: &str, date: &str, region: &str, service_name: &str) -> Vec<u8> {
    let mut sg = Hmac256::new_varkey(&format!("AWS4{}", key).as_bytes()).unwrap();
    sg.input(date.as_bytes());
    let key = &sg.result().code()[..];

    let mut sg = Hmac256::new_varkey(key).unwrap();
    sg.input(region.as_bytes());
    let key = &sg.result().code()[..];

    let mut sg = Hmac256::new_varkey(key).unwrap();
    sg.input(service_name.as_bytes());
    let key = &sg.result().code()[..];

    let mut sg = Hmac256::new_varkey(key).unwrap();
    sg.input("aws4_request".as_bytes());
    let key = &sg.result().code();

    Vec::from(&key[..])
}
