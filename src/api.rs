use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use std::collections::HashMap;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

use crate::firecrawl_client::FirecrawlClient;
use crate::models::{ErrorResponse, SearchRequest, SearchResponse};
use crate::parser::parse_junkyard_page;
use crate::pick_n_pull::PicknPullSearch;

#[derive(Clone)]
pub struct AppState {
    pub firecrawl_client: Arc<FirecrawlClient>,
    pub pick_n_pull: Arc<PicknPullSearch>,
}

pub fn create_app(firecrawl_client: FirecrawlClient) -> Router {
    let state = AppState {
        firecrawl_client: Arc::new(firecrawl_client),
        pick_n_pull: Arc::new(PicknPullSearch::new()),
    };

    Router::new()
        .route("/search", post(search_vehicles))
        .route("/search", get(search_vehicles_get))
        .route("/health", get(health_check))
        .route("/supported-makes", get(get_supported_makes))
        .route("/supported-models", get(get_supported_models))
        .with_state(state)
        .layer(CorsLayer::permissive())
}

// POST /search - Main search endpoint
pub async fn search_vehicles(
    State(state): State<AppState>,
    Json(request): Json<SearchRequest>,
) -> Result<Json<SearchResponse>, (StatusCode, Json<ErrorResponse>)> {
    perform_search(state, request).await
}

// GET /search - Alternative GET endpoint for easier testing
pub async fn search_vehicles_get(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<SearchResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Parse query parameters into SearchRequest
    let make = params.get("make")
        .ok_or_else(|| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    success: false,
                    error: "Missing 'make' parameter".to_string(),
                }),
            )
        })?;

    let model = params.get("model")
        .ok_or_else(|| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    success: false,
                    error: "Missing 'model' parameter".to_string(),
                }),
            )
        })?;

    let year_min = params.get("year_min")
        .and_then(|s| s.parse::<u32>().ok())
        .ok_or_else(|| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    success: false,
                    error: "Missing or invalid 'year_min' parameter".to_string(),
                }),
            )
        })?;

    let year_max = params.get("year_max")
        .and_then(|s| s.parse::<u32>().ok())
        .ok_or_else(|| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    success: false,
                    error: "Missing or invalid 'year_max' parameter".to_string(),
                }),
            )
        })?;

    let zip_code = params.get("zip_code")
        .ok_or_else(|| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    success: false,
                    error: "Missing 'zip_code' parameter".to_string(),
                }),
            )
        })?;

    let distance = params.get("distance")
        .and_then(|s| s.parse::<u32>().ok());

    let request = SearchRequest {
        make: make.clone(),
        model: model.clone(),
        year_min,
        year_max,
        zip_code: zip_code.clone(),
        distance,
    };

    perform_search(state, request).await
}

async fn perform_search(
    state: AppState,
    request: SearchRequest,
) -> Result<Json<SearchResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Validate year range
    if request.year_min > request.year_max {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                success: false,
                error: "year_min cannot be greater than year_max".to_string(),
            }),
        ));
    }

    // Default distance to 50 miles if not provided
    let distance = request.distance.unwrap_or(50);

    // Generate search URL
    let search_url = match state.pick_n_pull.generate_search_url(
        &request.make,
        &request.model,
        &request.zip_code,
        distance,
        (request.year_min, request.year_max),
    ) {
        Ok(url) => url,
        Err(error) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    success: false,
                    error,
                }),
            ))
        }
    };

    // Crawl the webpage
    let crawl_response = match state.firecrawl_client.crawl_webpage(&search_url).await {
        Ok(response) => response,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    success: false,
                    error: format!("Failed to crawl webpage: {}", e),
                }),
            ))
        }
    };

    // Parse the response
    let vehicles = if let Some(data) = &crawl_response.data {
        if let Some(markdown) = &data.markdown {
            parse_junkyard_page(markdown, &search_url)
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };

    let total_found = vehicles.len();

    Ok(Json(SearchResponse {
        success: true,
        vehicles,
        search_params: request,
        total_found,
    }))
}

// GET /health - Health check endpoint
pub async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "junkyard-tracker-api",
        "timestamp": chrono::Utc::now()
    }))
}

// GET /supported-makes - Get list of supported makes
pub async fn get_supported_makes(
    State(state): State<AppState>,
) -> Json<serde_json::Value> {
    let makes = state.pick_n_pull.get_supported_makes();
    Json(serde_json::json!({
        "success": true,
        "makes": makes
    }))
}

// GET /supported-models?make=<make> - Get list of supported models for a make
pub async fn get_supported_models(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    let make = params.get("make")
        .ok_or_else(|| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    success: false,
                    error: "Missing 'make' parameter".to_string(),
                }),
            )
        })?;

    let models = state.pick_n_pull.get_supported_models_for_make(make);
    
    Ok(Json(serde_json::json!({
        "success": true,
        "make": make,
        "models": models
    })))
}