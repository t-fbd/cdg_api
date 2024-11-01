#![doc = include_str!("../README.md")]

pub mod endpoints;
pub mod url_builders;
pub mod param_models;
pub mod response_models;
pub mod cdg_client;
pub mod cdg_types;

pub use request_handlers::get_congress_data;
pub use cdg_client::CongressApiClient;

pub const BASE_URL: &str = "https://api.congress.gov/v3/";


/// Unwraps an `Option<String>` and returns the inner `String` or an empty string if `None`.
pub fn unwrap_option_string(opt: Option<String>) -> String {
    match opt {
        Some(s) => s,
        None => "".to_string()
    }
}

/// Unwraps an `Option<u32>` and returns the inner `u32` or 0 if `None`.
pub fn unwrap_option_u32(opt: Option<u32>) -> u32 {
    match opt {
        Some(i) => i,
        None => 0
    }
}

/// Unwraps an `Option<T>` and returns the inner `T` or the default value if `None`.
pub fn unwrap_option<T: Default>(opt: Option<T>) -> T {
    match opt {
        Some(t) => t,
        None => T::default()
    }
}


pub mod request_handlers {
//! # `request_handlers` Module
//! 
//! The `request_handlers` module provides utility functions for interacting with the US Congress API.
//! It includes methods for fetching data via HTTP requests and processing responses using external tools.
//! 
//! ## Functions
//! 
//! - **`get_congress_data`**: Sends an HTTP GET request to a specified URL and deserializes the JSON response into the desired type.
//! - **`curl_and_jq`**: Executes a `curl` command to retrieve data from a URL and processes the JSON output with `jq`.
//! 
//! ## Usage
//! 
//! ```rust
//! use cdg_api::request_handlers::{get_congress_data, curl_and_jq};
//! use cdg_api::response_models::BillsResponse;
//! 
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let url = "https://api.congress.gov/v3/bill/?format=json&limit=10&api_key=YOUR_API_KEY";
//!     
//!     // Fetch and deserialize data
//!     // Note:
//!     // In this example it will fail because the api key is not valid
//!     // otherwise it will return the data and attempt to deserialize it
//!     // into the BillsResponse struct
//!     let bills: BillsResponse = match get_congress_data(url) {
//!         Ok(data) => data,
//!         Err(err) => {
//!             eprintln!("Error: {}", err);
//!             BillsResponse::default()
//!         }
//!     };
//!     
//!     // Fetch and process data with jq
//!     // Note:
//!     // In this example it will fail because the api key is not valid
//!     // otherwise it will return the data and process it with jq
//!     // using the provided filter
//!     match curl_and_jq(url, ".bills[] | {number, title}") {
//!         Ok(_) => println!("Data processed successfully."),
//!         Err(err) => eprintln!("Error: {}", err),
//!     }
//!     
//!     Ok(())
//! }
//! ```
//! 
//! ## Summary
//! 
//! - **`get_congress_data`**: Ideal for Rust applications that require type-safe deserialization of API responses.
//! - **`curl_and_jq`**: Useful for command-line applications or scripts where external processing of JSON data is preferred.
//! - **Dependencies**: Ensure that `reqwest` and `serde` are included for `get_congress_data`, and that `curl` and `jq` are installed for `curl_and_jq`.


    use reqwest::blocking::Client;
    use serde::de::DeserializeOwned;
    use super::response_models::PrimaryResponse;

    /// Fetches data from the US Congress API and deserializes it into the specified response model.
    ///
    /// # Parameters
    ///
    /// - `url`: The API endpoint URL.
    ///
    /// # Returns
    ///
    /// - `Ok(T)`: The deserialized response data.
    /// - `Err`: An error if the request fails or deserialization fails.
    pub fn get_congress_data<T: PrimaryResponse + DeserializeOwned>(url: &str) -> Result<T, Box<dyn std::error::Error>> {
        let client = Client::new();
        let response = client.get(url).send()?;
        let data = response.json::<T>()?;
        Ok(data)
    }
    
    use std::io::Write;
    
    /// Executes a `curl` request to the given URL and processes the JSON output with `jq`.
    ///
    /// # Parameters
    ///
    /// - `url`: The API endpoint URL.
    /// - `jq_cmd`: A `jq` filter string to process the JSON output.
    ///
    /// # Returns
    ///
    /// - `Ok(())`: If the command executes successfully.
    /// - `Err`: If there is an error executing the commands or processing the data.
    ///
    /// # Notes
    ///
    /// - Requires `curl` and `jq` to be installed on your system.
    /// - Ensures that the `CDG_API_KEY` environment variable is set.
    pub fn curl_and_jq(url: &str, jq_cmd: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Check if jq is installed
        if std::process::Command::new("jq")
            .arg("--version")
            .output()
            .is_err()
        {
            return Err("jq is not installed. Please install jq and try again.".into());
        }
    
        // Check if curl is installed
        if std::process::Command::new("curl")
            .arg("--version")
            .output()
            .is_err()
        {
            return Err("curl is not installed. Please install curl and try again.".into());
        }
    
        // Execute curl command
        let curl_output = std::process::Command::new("curl")
            .arg(url)
            .output()?;
    
        if !curl_output.status.success() {
            return Err("Failed to execute curl command.".into());
        }
    
        let stdout = String::from_utf8(curl_output.stdout)?;
        let trimmed_output = stdout.trim();
    
        // Execute jq command
        let mut jq_process = std::process::Command::new("jq")
            .arg(jq_cmd)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::inherit())
            .spawn()?;
    
        {
            let stdin = jq_process.stdin.as_mut().ok_or("Failed to open stdin for jq")?;
            stdin.write_all(trimmed_output.as_bytes())?;
        }
    
        let jq_status = jq_process.wait()?;
        if !jq_status.success() {
            return Err("jq command failed.".into());
        }
    
        Ok(())
    }
}
