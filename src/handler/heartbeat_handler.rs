use crate::response::api_response::ApiSuccessResponse;
use axum::Json;
use tracing::info;
pub async fn heartbeat() -> Json<ApiSuccessResponse<String>> {
    info!("Handling heartbeat request");
    Json(ApiSuccessResponse::send("OK".to_string()))
}
