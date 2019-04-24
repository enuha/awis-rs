pub mod url_info {
    pub const ACTION : &str = "UrlInfo";

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

    pub const VALID_GROUPS: [&str; 14] = [RELATED_LINKS, CATEGORIES, RANK, CONTACT_INFO, RANK_BY_COUNTRY,
        USAGE_STATS, SPEED, LANGUAGE, OWNED_DOMAINS, LINKED_IN_COUNT,
        SITE_DATA, ADULT_CONTENT, TRAFFIC_DATA, CONTENT_DATA];
}

pub mod traffic_history {
    pub const ACTION : &str = "TrafficHistory";
    pub const HISTORY: &str = "History";
}

pub mod category_browse {
    pub const ACTION : &str = "CategoryBrowse";
    pub const CATEGORIES: &str = "Categories";
    pub const RELATED_CATEGORIES: &str = "RelatedCategories";
    pub const LANGUAGE_CATEGORIES: &str = "LanguageCategories";
    pub const LETTER_BARS: &str = "LetterBars";

    pub const VALID_GROUPS: [&str; 4] = [CATEGORIES, RELATED_CATEGORIES, LANGUAGE_CATEGORIES, LETTER_BARS];
}

pub mod category_listing {
    pub const ACTION : &str = "CategoryListing";
    pub const LISTINGS: &str = "Listings";
}

pub mod sites_linking_in {
    pub const ACTION : &str = "SitesLinkingIn";
    pub const SITES_LINKING_IN: &str = "SitesLinkingIn";
}