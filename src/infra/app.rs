use crate::{
    adapters::{
        http::{app_state::AppState, routes::api_routes::api_routes},
        openapi::ApiDoc,
    },
    infra::setup::init_tracing,
};
use axum::{Router, http};
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub fn create_app(app_state: AppState) -> Router {
    init_tracing();

    let mut router = Router::new();

    if app_state.config.enable_swagger {
        router = router
            .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));
    }

    router.merge(api_routes()).with_state(app_state).layer(
        TraceLayer::new_for_http().make_span_with(|request: &http::Request<_>| {
            tracing::info_span!(
                "http-request",
                method = %request.method(),
                uri = %request.uri(),
                version = ?request.version(),
            )
        }),
    )
}
