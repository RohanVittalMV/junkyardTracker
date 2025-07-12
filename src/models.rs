use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JunkyardItem {
    pub id: String,
    pub make: String,
    pub model: String,
    pub year: Option<u32>,
    pub location: Option<String>,
    pub availability: bool,
    pub added_date: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchRequest {
    pub make: String,
    pub model: String,
    pub year_min: u32,
    pub year_max: u32,
    pub zip_code: String,
    pub distance: Option<u32>, // Optional, defaults to 50 miles
}

#[derive(Debug, Serialize)]
pub struct SearchResponse {
    pub success: bool,
    pub vehicles: Vec<JunkyardItem>,
    pub search_params: SearchRequest,
    pub total_found: usize,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
}