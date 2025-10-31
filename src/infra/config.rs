#[derive(Clone)]
pub struct AppConfig {
    pub server_port: u16,
    pub enable_swagger: bool,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let server_port = std::env::var("SERVER_PORT").expect("SERVER_PORT must be set");

        let enable_swagger = std::env::var("ENABLE_SWAGGER")
            .unwrap_or_else(|_| "true".to_string())
            .parse::<bool>()
            .expect("ENABLE_SWAGGER must be a boolean (true/false)");

        Self {
            server_port: server_port.parse().expect("SERVER_PORT must be a number"),
            enable_swagger,
        }
    }
}
