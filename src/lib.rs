//!# Rust API for the [US Congress API](https://api.congress.gov/)
//!
//!## Overview
//!
//!This library provides a Rust interface for the [US Congress API](https://api.congress.gov/). It
//!allows users to interact with the API endpoints, construct URLs, and parse the responses into
//!Rust data structures.
//!
//!## Features
//!
//!## Usage
//!
//!## Examples

pub mod endpoints;
pub mod url_builders;
pub mod param_models;

pub use endpoints::*;
pub const BASE_URL: &str = "https://api.congress.gov/v3/";


use std::io::Write;
pub fn curl_and_jq(url: &str, jq_cmd: &str) -> Result<(), Box<dyn std::error::Error>> {
    // check if jq is installed, check if curl is installed
    // if not, return an error
    std::process::Command::new("jq")
        .arg("--version")
        .output()
        .expect("jq is not installed. Please install jq and try again.");

    std::process::Command::new("curl")
        .arg("--version")
        .output()
        .expect("curl is not installed. Please install curl and try again.");

    let output = std::process::Command::new("curl")
        .arg(url)
        .output()
        .expect("Failed to execute command");

    let output = std::str::from_utf8(&output.stdout).unwrap();
    let output = output.trim();

    let _ = std::process::Command::new("jq")
        .arg(jq_cmd)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::inherit())
        .spawn()
        .expect("Failed to execute command")
        .stdin
        .unwrap()
        .write_all(output.as_bytes());

    Ok(())

}
