use axum::{
    routing::{delete, get, post, put},
    Router,
};

use super::controller::{self, SharedUserService};
use super::middleware;

pub fn routes(service: SharedUserService) -> Router {
    Router::new()
        .route(
            "/v1/user/:id",
            get(controller::find_by_id).layer(axum::middleware::from_fn(middleware::find_by_id)),
        )
        .route("/v1/user", get(controller::list_all))
        .route("/v1/user", post(controller::create))
        .route("/v1/user/:id", put(controller::update))
        .route("/v1/user/:id", delete(controller::remove))
        .with_state(service)
}
