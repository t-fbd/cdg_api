//! ### `client` Module
//!
//! The `client` module provides the [`CongressApiClient`] struct, which serves as the primary interface for interacting with the US Congress API. It handles API key management, URL construction, making HTTP requests, and deserializing responses.
//!
//! #### Usage Example
//!
//! ```rust
//! use cdg_api::CongressApiClient;
//! use cdg_api::endpoints::{Endpoints, NewEndpoint};
//! use cdg_api::param_models::MemberListParams;
//! use cdg_api::cdg_types::FormatType;
//! use cdg_api::response_models::MembersResponse;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = CongressApiClient::new(None)?; // Uses API key from environment
//!
//!     let params = MemberListParams::default()
//!         .format(FormatType::Json)
//!         .limit(10)
//!         .current_member(true);
//!
//!     let endpoint = Endpoints::new_member_list(params);
//!     let response: MembersResponse = client.fetch(endpoint)?;
//!
//!     for member in response.members {
//!         println!(
//!             "{}, {}, {}",
//!             member.name.unwrap_or("".to_string()),
//!             member.state.unwrap_or("".to_string()),
//!             member.party_name.unwrap_or("".to_string())
//!         );
//!     }
//!
//!     Ok(())
//! }
//! ```

use crate::{endpoints::Endpoints, response_models::PrimaryResponse, url_builders::generate_url};
use reqwest::blocking::Client;
use serde::de::DeserializeOwned;
use std::env;
use std::error::Error;
use std::fmt;

/// A client for interacting with the US Congress API.
pub struct CongressApiClient {
    api_key: String,
    client: Client,
}

impl CongressApiClient {
    /// Creates a new instance of [`CongressApiClient`].
    ///
    /// # Parameters
    ///
    /// - [`api_key`]: An optional API key. If [`None`], the client will attempt to read the [`CDG_API_KEY`] environment variable.
    ///
    /// # Returns
    ///
    /// - `Ok(CongressApiClient)`: A new client instance.
    /// - [`Err`]: If the API key is not provided and not found in the environment.
    pub fn new(api_key: Option<String>) -> Result<Self, Box<dyn Error>> {
        let api_key = match api_key {
            Some(key) => key,
            None => env::var("CDG_API_KEY")
                .map_err(|_| "CDG_API_KEY environment variable not set. Alternatively, provide an API key as an argument.")?,
        };

        Ok(Self {
            api_key,
            client: Client::new(),
        })
    }

    /// Fetches data from the US Congress API for a given endpoint.
    ///
    /// # Parameters
    ///
    /// - [`endpoint`]: The API endpoint variant.
    ///
    /// - [`T`]: The type of the response data. This type must implement [`PrimaryResponse`] and
    /// [`DeserializeOwned`].
    ///
    /// # Returns
    ///
    /// - Result<T, ApiClientError>: The fetched data, deserialized into the appropriate response
    /// type.
    ///
    /// # Errors
    /// - `ApiClientError::Http`: If an HTTP error occurs.
    /// - `ApiClientError::Deserialization`: If an error occurs during deserialization.
    /// - `ApiClientError::Url`: If an error occurs while building the URL.
    /// - `ApiClientError::EnvVar`: If the API key is not found in the environment.
    pub fn fetch<T: PrimaryResponse + DeserializeOwned>(
        &self,
        endpoint: Endpoints,
    ) -> Result<T, ApiClientError> {
        let url = generate_url(endpoint, &self.api_key);
        let response = self
            .client
            .get(&url)
            .send()
            .map_err(ApiClientError::Http)?
            .error_for_status()
            .map_err(|e| ApiClientError::Http(e))?; // Map HTTP errors

        let data = response
            .json::<T>()
            .map_err(|e: reqwest::Error| ApiClientError::Http(e))?;
        Ok(data)
    }
}

/// Custom error type for [`CongressApiClient`].
#[derive(Debug)]
pub enum ApiClientError {
    Http(reqwest::Error),
    Url(String),
    Deserialization(serde_json::Error),
    EnvVar(String),
}

impl fmt::Display for ApiClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiClientError::Http(e) => write!(f, "HTTP error: {}", e),
            ApiClientError::Url(e) => write!(f, "URL error: {}", e),
            ApiClientError::Deserialization(e) => write!(f, "Deserialization error: {}", e),
            ApiClientError::EnvVar(e) => write!(f, "Environment variable error: {}", e),
        }
    }
}

impl Error for ApiClientError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ApiClientError::Http(e) => Some(e),
            ApiClientError::Deserialization(e) => Some(e),
            _ => None,
        }
    }
}
