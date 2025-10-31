use crate::adapters::http::app_state::AppState;
use crate::adapters::http::routes::health::health_check;
use crate::adapters::http::routes::pokemon::{pokemon_api, pokemon_translated_api};
use axum::Router;
use axum::routing::get;

pub fn api_routes() -> Router<AppState> {
    Router::new()
        .route("/health", get(health_check))
        .route("/pokemon/{pokemon_name}", get(pokemon_api))
        .route(
            "/pokemon/translated/{pokemon_name}",
            get(pokemon_translated_api),
        )
}
