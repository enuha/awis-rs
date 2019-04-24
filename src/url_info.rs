use serde::{Serialize, Deserialize, Deserializer};
use serde_xml_rs::from_str;


#[derive(Debug, Serialize, Deserialize)]
struct OperationRequest {
    #[serde(rename = "RequestId")]
    request_id: String
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct SiteData {
    #[serde(deserialize_with = "string_deserializer", default)]
    title: Option<String>,
    #[serde(deserialize_with = "string_deserializer", default)]
    description: Option<String>,
    #[serde(deserialize_with = "string_deserializer", default)]
    online_since: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Speed {
    #[serde(deserialize_with = "u64_deserializer", default)]
    median_load_time: Option<u64>,
    #[serde(deserialize_with = "u64_deserializer", default)]
    percentile: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct OwnedDomain {
    #[serde(deserialize_with = "string_deserializer", default)]
    domain: Option<String>,
    #[serde(deserialize_with = "string_deserializer", default)]
    title: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Language {
    #[serde(deserialize_with = "string_deserializer", default)]
    locale: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ContentData {
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
struct Alexa {
    content_data: Option<ContentData>,
    traffic_data: Option<TrafficData>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct UrlInfoResult {
    alexa: Alexa
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ResponseStatus {
    status_code: String
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct NestedUrlInfoResponse {
    operation_request: OperationRequest,
    url_info_result: UrlInfoResult,
    response_status: ResponseStatus,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct UrlInfoResponse {
    response: NestedUrlInfoResponse
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct TrafficData {
    #[serde(deserialize_with = "string_deserializer", default)]
    data_url: Option<String>,
    #[serde(deserialize_with = "string_deserializer", default)]
    asin: Option<String>,
    #[serde(deserialize_with = "u64_deserializer", default)]
    rank: Option<u64>,
    usage_statistics: Option<Vec<UsageStatistic>>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct TimeRange {
    #[serde(deserialize_with = "u64_deserializer", default)]
    days: Option<u64>,
    #[serde(deserialize_with = "u64_deserializer", default)]
    months: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Rank {
    #[serde(deserialize_with = "u64_deserializer", default)]
    value: Option<u64>,
    #[serde(deserialize_with = "u64_deserializer", default)]
    delta: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct UsageStatistic {
    time_range: Option<TimeRange>,
    rank: Option<Rank>,
}
