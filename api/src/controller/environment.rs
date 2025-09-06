use axum::http::{HeaderValue, Method};
use tower_http::cors::CorsLayer;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Environment {
    Development,
    Production,
    Testing,
}

impl Environment {
    pub fn from_env() -> Self {
        match std::env::var("RUST_ENV").as_deref() {
            Ok("development") | Ok("dev") => Environment::Development,
            Ok("testing") | Ok("test") => Environment::Testing,
            _ => Environment::Production,
        }
    }
    
    pub fn needs_cors(&self) -> bool {
        matches!(self, Environment::Testing | Environment::Development)
    }
    
    pub fn cors_origins(&self) -> Vec<&'static str> {
        match self {
            Self::Development => vec!["http://localhost:3000", "http://localhost:5173"],
            Self::Testing => vec!["http://localhost:3001"],
            Self::Production => vec![],
        }
    }
    
    pub fn default_port(self) -> u16 {
        match self {
            Self::Development => 3000,
            Self::Testing => 3001,
            Self::Production => 8080,
        }
    }
    
    pub fn bind_address(&self) -> String {
        let port = std::env::var("PORT")
            .ok()
            .and_then(|p| p.parse::<u16>().ok())
            .unwrap_or_else(|| self.clone().default_port());
        
        match self {
            Self::Development => format!("127.0.0.1:{}", port),
            _ => format!("0.0.0.0:{}", port)
        }
    }
    
    pub fn tracing_level(&self) -> &'static str {
        match self {
            Self::Development => "debug,tower_http=debug,axum=debug",
            Self::Testing => "debug",
            Self::Production => "info",
        }
    }
}

pub fn create_cors_layer(env: &Environment) -> Option<CorsLayer> {
    if !env.needs_cors() {
        return None;
    }
    
    let origins: Result<Vec<HeaderValue>, _> = env.cors_origins()
        .iter()
        .map(|origin| origin.parse::<HeaderValue>())
        .collect();
    
    match origins {
        Ok(origins) => Some(
            CorsLayer::new()
                .allow_origin(origins)
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        ),
        Err(_) => None,
    }
}