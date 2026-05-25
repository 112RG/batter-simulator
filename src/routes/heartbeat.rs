use axum::{Router, routing::get};

use crate::handler::heartbeat_handler;

pub fn routes() -> Router {
    
    Router::new().route("/heartbeat", get(heartbeat_handler::heartbeat))
}
