use serde::{Serialize, Deserialize};

use crate::serde_utils::*;

pub mod groups {
    pub const RELATED_LINKS: &str = "RelatedLinks";
    pub const CATEGORIES: &str = "Categories";
    pub const RANK: &str = "Rank";
    pub const CONTACT_INFO: &str = "ContactInfo";
    pub const RANK_BY_COUNTRY: &str = "RankByCountry";
    pub const USAGE_STATS: &str = "UsageStats";
    pub const SPEED: &str = "Speed";
    pub const LANGUAGE: &str = "Language";
    pub const OWNED_DOMAINS: &str = "OwnedDomains";
    pub const LINKED_IN_COUNT: &str = "LinksInCount";
    pub const SITE_DATA: &str = "SiteData";
    pub const ADULT_CONTENT: &str = "AdultContent";

    pub const TRAFFIC_DATA: &str = "TrafficData";
    pub const CONTENT_DATA: &str = "ContentData";
}

pub const ACTION: &str = "UrlInfo";

pub const VALID_GROUPS: [&str; 14] = [groups::RELATED_LINKS, groups::CATEGORIES, groups::RANK,
    groups::CONTACT_INFO, groups::RANK_BY_COUNTRY, groups::USAGE_STATS, groups::SPEED,
    groups::LANGUAGE, groups::OWNED_DOMAINS, groups::LINKED_IN_COUNT, groups::SITE_DATA,
    groups::ADULT_CONTENT, groups::TRAFFIC_DATA, groups::CONTENT_DATA];

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UrlInfoResponse {
    response: NestedUrlInfoResponse
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NestedUrlInfoResponse {
    operation_request: OperationRequest,
    url_info_result: UrlInfoResult,
    response_status: ResponseStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationRequest {
    #[serde(rename = "RequestId")]
    request_id: String
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SiteData {
    #[serde(deserialize_with = "string_deserializer", default)]
    title: Option<String>,
    #[serde(deserialize_with = "string_deserializer", default)]
    description: Option<String>,
    #[serde(deserialize_with = "string_deserializer", default)]
    online_since: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Speed {
    #[serde(deserialize_with = "u64_deserializer", default)]
    median_load_time: Option<u64>,
    #[serde(deserialize_with = "u64_deserializer", default)]
    percentile: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct OwnedDomain {
    #[serde(deserialize_with = "string_deserializer", default)]
    domain: Option<String>,
    #[serde(deserialize_with = "string_deserializer", default)]
    title: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Language {
    #[serde(deserialize_with = "string_deserializer", default)]
    locale: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContentData {
    #[serde(deserialize_with = "string_deserializer", default)]
    data_url: Option<String>,
    #[serde(deserialize_with = "string_deserializer", default)]
    asin: Option<String>,
    site_data: Option<SiteData>,
    speed: Option<Speed>,
    #[serde(deserialize_with = "string_deserializer", default)]
    adult_content: Option<String>,
    language: Option<Language>,
    #[serde(deserialize_with = "u64_deserializer", default)]
    links_in_count: Option<u64>,
    owned_domains: Option<Vec<OwnedDomain>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Alexa {
    content_data: Option<ContentData>,
    traffic_data: Option<TrafficData>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UrlInfoResult {
    alexa: Alexa
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResponseStatus {
    status_code: String
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TrafficData {
    #[serde(deserialize_with = "string_deserializer", default)]
    data_url: Option<String>,
    #[serde(deserialize_with = "string_deserializer", default)]
    asin: Option<String>,
    #[serde(deserialize_with = "u64_deserializer", default)]
    rank: Option<u64>,
    usage_statistics: Option<Vec<UsageStatistic>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UsageStatistic {
    time_range: Option<TimeRange>,
    rank: Option<DeltaValue>,
    reach: Option<Reach>,
    page_views: Option<PageViews>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TimeRange {
    #[serde(deserialize_with = "u64_deserializer", default)]
    days: Option<u64>,
    #[serde(deserialize_with = "u64_deserializer", default)]
    months: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeltaValue {
    #[serde(deserialize_with = "u64_deserializer", default)]
    value: Option<u64>,
    #[serde(deserialize_with = "string_deserializer", default)]
    delta: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Reach {
    rank: Option<DeltaValue>,
    per_million: Option<DeltaValue>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PageViews {
    rank: Option<DeltaValue>,
    per_million: Option<DeltaValue>,
    per_user: Option<DeltaValue>,
}
