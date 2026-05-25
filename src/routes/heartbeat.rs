use axum::{Router, routing::get};

use crate::handler::heartbeat_handler;

pub fn routes() -> Router {
    let router = Router::new().route("/heartbeat", get(heartbeat_handler::heartbeat));
    router
}
