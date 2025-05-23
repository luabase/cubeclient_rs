#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1LoadRequestQuery {
    #[serde(rename = "measures", skip_serializing_if = "Option::is_none")]
    pub measures: Option<Vec<String>>,
    #[serde(rename = "dimensions", skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<String>>,
    #[serde(rename = "segments", skip_serializing_if = "Option::is_none")]
    pub segments: Option<Vec<String>>,
    #[serde(rename = "timeDimensions", skip_serializing_if = "Option::is_none")]
    pub time_dimensions: Option<Vec<crate::models::V1LoadRequestQueryTimeDimension>>,
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<serde_json::Value>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    #[serde(rename = "filters", skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<crate::models::V1LoadRequestQueryFilterItem>>,
    #[serde(rename = "renewQuery", skip_serializing_if = "Option::is_none")]
    pub renew_query: Option<bool>,
    #[serde(rename = "forceNoCache", skip_serializing_if = "Option::is_none")]
    pub force_no_cache: Option<bool>,
}

impl V1LoadRequestQuery {
    pub fn new() -> V1LoadRequestQuery {
        V1LoadRequestQuery {
            measures: None,
            dimensions: None,
            segments: None,
            time_dimensions: None,
            order: None,
            limit: None,
            offset: None,
            filters: None,
            renew_query: None,
            force_no_cache: None,
        }
    }
}
