use crate::routes::heartbeat;
use axum::Router;
use axum::routing::IntoMakeService;
use tower_http::trace::TraceLayer;

pub fn routes() -> IntoMakeService<Router> {
    let app_router = Router::new()
        .nest("/api", heartbeat::routes())
        .layer(TraceLayer::new_for_http());

    app_router.into_make_service()
}
