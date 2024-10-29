#![doc = include_str!("../README.md")]
//! # Rust API for the US Congress API

pub mod endpoints;
pub mod url_builders;
pub mod param_models;
pub mod response_models;

pub use endpoints::*;
pub const BASE_URL: &str = "https://api.congress.gov/v3/";

use reqwest::blocking::Client;
use response_models::PrimaryResponse;
use serde::de::DeserializeOwned;

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
