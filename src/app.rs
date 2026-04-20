use std::sync::Arc;

use axum::Router;

use crate::config::Env;
use crate::plugins::register_plugins;
use crate::v1::modules::user::{
    controller::SharedUserService, routes as user_routes, service::UserService,
};

pub fn build_router(env: &Env) -> Router {
    let user_service: SharedUserService = Arc::new(UserService::new());
    let user_router = user_routes::routes(user_service);

    let api_router = Router::new().nest(&env.strip_prefix.path, user_router);

    register_plugins(api_router, env)
}
