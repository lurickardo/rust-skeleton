use rust_skeleton::config::env::{load_env, sanitized_app_name};

use crate::common::ENV_LOCK;

#[test]
fn sanitized_name_removes_hyphens() {
    assert_eq!(sanitized_app_name(), "rustskeleton");
}

#[test]
fn load_env_defaults_when_missing() {
    let _guard = ENV_LOCK.lock().unwrap();
    std::env::remove_var("PORT");
    std::env::remove_var("APP_ENVIRONMENT");
    std::env::remove_var("USE_ROUTE_PREFIX");
    let env = load_env();
    assert_eq!(env.app.port, 3000);
    assert_eq!(env.app.environment, "DEV");
    assert_eq!(env.strip_prefix.path, "/api/rustskeleton");
    assert_eq!(env.plugins.swagger.base_path, "/");
}

#[test]
fn load_env_uses_prefix_when_enabled() {
    let _guard = ENV_LOCK.lock().unwrap();
    std::env::set_var("USE_ROUTE_PREFIX", "true");
    let env = load_env();
    assert_eq!(env.plugins.swagger.base_path, "/api/rustskeleton/");
    std::env::remove_var("USE_ROUTE_PREFIX");
}
