use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::config::Env;
use crate::v1::modules::user;

#[derive(OpenApi)]
#[openapi(
    paths(
        user::controller::find_by_id,
        user::controller::list_all,
        user::controller::create,
        user::controller::update,
        user::controller::remove,
    ),
    components(schemas(
        user::schema::UserResponse,
        user::dto::CreateUserDto,
        user::dto::UpdateUserDto,
        user::schema::DeleteMessage,
    )),
    tags(
        (name = "v1", description = "User API v1")
    )
)]
pub struct ApiDoc;

pub fn routes(env: &Env) -> Router {
    let docs_path = format!("{}/docs", env.strip_prefix.path);
    Router::new().merge(SwaggerUi::new(docs_path).url("/api-docs/openapi.json", ApiDoc::openapi()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn openapi_document_is_generated() {
        let doc = ApiDoc::openapi();
        let json = serde_json::to_string(&doc).unwrap();
        assert!(json.contains("/v1/user"));
    }
}
