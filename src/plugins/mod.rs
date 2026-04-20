pub mod cors;
pub mod healthcheck;
pub mod swagger;
pub mod validation;

use axum::Router;

use crate::config::Env;

pub fn register_plugins(router: Router, env: &Env) -> Router {
    let router = router.merge(healthcheck::routes(env)).layer(cors::layer());

    if env.app.environment.to_uppercase() != "PRD" {
        router.merge(swagger::routes(env))
    } else {
        router
    }
}
