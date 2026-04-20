use std::env;

#[derive(Debug, Clone)]
pub struct AppSection {
    pub port: u16,
    pub environment: String,
}

#[derive(Debug, Clone)]
pub struct SwaggerSection {
    pub base_path: String,
}

#[derive(Debug, Clone)]
pub struct PluginsSection {
    pub swagger: SwaggerSection,
}

#[derive(Debug, Clone)]
pub struct StripPrefixSection {
    pub path: String,
}

#[derive(Debug, Clone)]
pub struct DatabaseSection {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct Env {
    pub app: AppSection,
    pub plugins: PluginsSection,
    pub strip_prefix: StripPrefixSection,
    pub database: DatabaseSection,
}

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");

pub fn sanitized_app_name() -> String {
    APP_NAME.replace('-', "")
}

pub fn load_env() -> Env {
    let _ = dotenvy::dotenv();

    let port: u16 = env::var("PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(3000);

    let environment = env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "DEV".to_string());

    let use_prefix = env::var("USE_ROUTE_PREFIX")
        .map(|v| v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);

    let sanitized = sanitized_app_name();
    let strip_prefix_path = format!("/api/{sanitized}");

    let swagger_base_path = if use_prefix {
        format!("/api/{sanitized}/")
    } else {
        "/".to_string()
    };

    Env {
        app: AppSection { port, environment },
        plugins: PluginsSection {
            swagger: SwaggerSection {
                base_path: swagger_base_path,
            },
        },
        strip_prefix: StripPrefixSection {
            path: strip_prefix_path,
        },
        database: DatabaseSection {
            name: env::var("DB_NAME").unwrap_or_default(),
            url: env::var("DB_URL").unwrap_or_default(),
        },
    }
}
