pub mod traffic_history {
    pub const ACTION: &str = "TrafficHistory";
    pub const HISTORY: &str = "History";
}

pub mod category_browse {
    pub const ACTION: &str = "CategoryBrowse";
    pub const CATEGORIES: &str = "Categories";
    pub const RELATED_CATEGORIES: &str = "RelatedCategories";
    pub const LANGUAGE_CATEGORIES: &str = "LanguageCategories";
    pub const LETTER_BARS: &str = "LetterBars";

    pub const VALID_GROUPS: [&str; 4] = [
        CATEGORIES,
        RELATED_CATEGORIES,
        LANGUAGE_CATEGORIES,
        LETTER_BARS,
    ];
}

pub mod category_listing {
    pub const ACTION: &str = "CategoryListing";
    pub const LISTINGS: &str = "Listings";
}

pub mod sites_linking_in {
    pub const ACTION: &str = "SitesLinkingIn";
    pub const SITES_LINKING_IN: &str = "SitesLinkingIn";
}
