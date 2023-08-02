//! This module contains various functions and submodules covering the
//! [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2).

/// The base URL for all API requests.
pub const API_URL: &str = "https://kztimerglobal.com/api/v2";

/// The URL for the API's SwaggerUI website.
pub const SWAGGER_URL: &str = "https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2";

pub mod health;
pub use health::{healthcheck, Health};
