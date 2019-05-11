use serde::{Deserialize, Serialize};

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

pub const VALID_GROUPS: [&str; 14] = [
    groups::RELATED_LINKS,
    groups::CATEGORIES,
    groups::RANK,
    groups::CONTACT_INFO,
    groups::RANK_BY_COUNTRY,
    groups::USAGE_STATS,
    groups::SPEED,
    groups::LANGUAGE,
    groups::OWNED_DOMAINS,
    groups::LINKED_IN_COUNT,
    groups::SITE_DATA,
    groups::ADULT_CONTENT,
    groups::TRAFFIC_DATA,
    groups::CONTENT_DATA,
];

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UrlInfoResponse {
    pub response: NestedUrlInfoResponse,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NestedUrlInfoResponse {
    pub operation_request: OperationRequest,
    pub url_info_result: UrlInfoResult,
    pub response_status: ResponseStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationRequest {
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SiteData {
    #[serde(deserialize_with = "string_deserializer", default)]
    pub title: Option<String>,
    #[serde(deserialize_with = "string_deserializer", default)]
    pub description: Option<String>,
    #[serde(deserialize_with = "string_deserializer", default)]
    pub online_since: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Speed {
    #[serde(deserialize_with = "u64_deserializer", default)]
    pub median_load_time: Option<u64>,
    #[serde(deserialize_with = "u64_deserializer", default)]
    pub percentile: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct OwnedDomain {
    #[serde(deserialize_with = "string_deserializer", default)]
    pub domain: Option<String>,
    #[serde(deserialize_with = "string_deserializer", default)]
    pub title: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Language {
    #[serde(deserialize_with = "string_deserializer", default)]
    pub locale: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContentData {
    #[serde(deserialize_with = "string_deserializer", default)]
    pub data_url: Option<String>,
    #[serde(deserialize_with = "string_deserializer", default)]
    pub asin: Option<String>,
    pub site_data: Option<SiteData>,
    pub speed: Option<Speed>,
    #[serde(deserialize_with = "string_deserializer", default)]
    pub adult_content: Option<String>,
    pub language: Option<Language>,
    #[serde(deserialize_with = "u64_deserializer", default)]
    pub links_in_count: Option<u64>,
    pub owned_domains: Option<Vec<OwnedDomain>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Alexa {
    pub content_data: Option<ContentData>,
    pub traffic_data: Option<TrafficData>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UrlInfoResult {
    pub alexa: Alexa,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResponseStatus {
    pub status_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TrafficData {
    #[serde(deserialize_with = "string_deserializer", default)]
    pub data_url: Option<String>,
    #[serde(deserialize_with = "string_deserializer", default)]
    pub asin: Option<String>,
    #[serde(deserialize_with = "u64_deserializer", default)]
    pub rank: Option<u64>,
    pub usage_statistics: Option<UsageStatistics>,
    pub contributing_subdomains: Option<ContributingSubdomains>,
    pub rank_by_country: Option<RankByCountry>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UsageStatistics {
    pub usage_statistic: Option<Vec<UsageStatistic>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContributingSubdomains {
    pub contributing_subdomain: Option<Vec<ContributingSubdomain>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UsageStatistic {
    pub time_range: Option<TimeRange>,
    pub rank: Option<IntDeltaValue>,
    pub reach: Option<Reach>,
    pub page_views: Option<PageViews>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContributingSubdomain {
    #[serde(deserialize_with = "string_deserializer", default)]
    pub data_url: Option<String>,
    pub time_range: Option<TimeRange>,
    pub reach: Option<DomainReach>,
    pub page_views: Option<DomainPageViews>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TimeRange {
    #[serde(deserialize_with = "u64_deserializer", default)]
    pub days: Option<u64>,
    #[serde(deserialize_with = "u64_deserializer", default)]
    pub months: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FloatDeltaValue {
    #[serde(deserialize_with = "f64_deserializer", default)]
    pub value: Option<f64>,
    #[serde(deserialize_with = "f64_deserializer", default)]
    pub delta: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase", serialize = "camelCase"))]
pub struct IntDeltaValue {
    #[serde(deserialize_with = "u64_deserializer", default)]
    pub value: Option<u64>,
    #[serde(deserialize_with = "f64_deserializer", default)]
    pub delta: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Reach {
    pub rank: Option<IntDeltaValue>,
    pub per_million: Option<FloatDeltaValue>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PageViews {
    pub rank: Option<IntDeltaValue>,
    pub per_million: Option<FloatDeltaValue>,
    pub per_user: Option<FloatDeltaValue>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DomainReach {
    #[serde(deserialize_with = "f64_deserializer", default)]
    pub percentage: Option<f64>,
    pub per_million: Option<FloatDeltaValue>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DomainPageViews {
    #[serde(deserialize_with = "f64_deserializer", default)]
    pub percentage: Option<f64>,
    #[serde(deserialize_with = "f64_deserializer", default)]
    pub per_user: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RankByCountry {
    pub country: Option<Vec<Country>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Country {
    #[serde(deserialize_with = "string_deserializer", default)]
    pub code: Option<String>,
    #[serde(deserialize_with = "u64_deserializer", default)]
    pub rank: Option<u64>,
    pub contribution: Option<CountryContribution>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CountryContribution {
    #[serde(deserialize_with = "f64_deserializer", default)]
    pub page_views: Option<f64>,
    #[serde(deserialize_with = "f64_deserializer", default)]
    pub users: Option<f64>,
}
