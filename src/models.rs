use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JunkyardItem {
    pub id: String,
    pub make: String,
    pub model: String,
    pub location: Option<String>,
    pub availability: bool,
    pub added_date: chrono::DateTime<chrono::Utc>,
}