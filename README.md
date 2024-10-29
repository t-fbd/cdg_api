# CDG API

![Crates.io](https://img.shields.io/crates/v/cdg_api)
![MIT License](https://img.shields.io/crates/l/cdg_api)

A Rust library for interacting with the [US Congress API](https://api.congress.gov/).  
Easily construct URLs, interact with various API endpoints, and parse responses into Rust data structures.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
  - [Setting Up](#setting-up)
  - [Creating Endpoints](#creating-endpoints)
  - [Generating URLs](#generating-urls)
- [Examples](#examples)
  - [Fetching a Bill](#fetching-a-bill)
  - [Retrieving Committee Information](#retrieving-committee-information)
- [API Endpoints](#api-endpoints)
- [Common Parameters](#common-parameters)
- [License](#license)

## Features

- **Comprehensive Coverage**: Supports a wide range of US Congress API endpoints including bills, laws, amendments, committees, hearings, and more.
- **Easy URL Construction**: Simplifies the process of constructing API request URLs with customizable parameters.
- **Serialization & Deserialization**: Utilizes `serde` for efficient JSON serialization and deserialization.
- **Extensible Design**: Easily add new endpoints or extend existing functionality.

## Installation

Add `cdg_api` to your project's `Cargo.toml`:

```toml
[dependencies]
cdg_api = "0.1.0"
```
~OR~

Add from cargo:

```bash
cargo add cdg_api
```
~OR~

Clone the repository and add it to your project:

```bash
git clone
```
and add it to your `Cargo.toml`:

```toml
[dependencies]
cdg_api = { path = "path/to/cdg_api" }
```


This project depends on the following crates:

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## Usage

### Setting Up

First, ensure you have an API key from [api.congress.gov](https://api.congress.gov/). Set it as an environment variable named `CDG_API_KEY`.

```bash
export CDG_API_KEY=your_api_key_here
```

### Creating Endpoints

Use the `Endpoints` enum to specify the desired API endpoint with the necessary parameters.

```rust
use cdg_api::{Endpoints, BillType, CommonParams};

fn main() {
    // Example: Create a bill endpoint
    let params = CommonParams {
        format: Some(cdg_api::Format::Json),
        offset: Some(0),
        limit: Some(10),
        from_date_time: None,
        to_date_time: None,
        sort: Some(cdg_api::Sort::DateDesc),
    };

    let bill_endpoint = Endpoints::new_bill(
        Some(117),           // Congress number
        Some(BillType::Hr),  // Bill type
        Some("1".to_string()), // Bill number
        Some(cdg_api::BillOption::Text), // Bill option
        Some(params),
    );

    let url = bill_endpoint.to_url();
    println!("Generated URL: {}", url);
}
```

### Generating URLs

Once you've defined an endpoint, use the `to_url` method to generate the complete API request URL.

```rust
let url = bill_endpoint.to_url();
// Use `reqwest` or any HTTP client to make the request
```

## Examples

### Fetching a Bill

```rs
use cdg_api::{Endpoints, BillType, BillOption, CommonParams};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up common parameters
    let params = CommonParams {
        format: Some(cdg_api::Format::Json),
        limit: Some(5),
        ..CommonParams::new()
    };

    // Create bill endpoint
    let bill_endpoint = Endpoints::new_bill(
        Some(117),
        Some(BillType::Hr),
        Some("1".to_string()),
        Some(BillOption::Text),
        Some(params),
    );

    // Generate URL
    let url = bill_endpoint.to_url();
    println!("Request URL: {}", url);

    // Make the HTTP request

    Ok(())
}
```

### Retrieving Committee Information

```rust
use cdg_api::{ChamberType, CommitteeOption, CommonParams, Endpoints, NewEndpoint};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up common parameters
    let params = CommonParams {
        format: Some(cdg_api::Format::Json),
        limit: Some(10),
        ..CommonParams::new()
    };

    // Create committee endpoint
    let committee_endpoint = Endpoints::new_committee(
        Some(ChamberType::Senate),
        Some(118),
        None,
        Some(CommitteeOption::Bills),
        Some(params),
    );

    // Generate URL
    let url = committee_endpoint.to_url();
    println!("Request URL: {}", url);

    // Make the HTTP request

    Ok(())
}
```

## API Endpoints

The `Endpoints` enum covers a variety of US Congress API endpoints. Below is a non-exhaustive list:

- **Bill**: Retrieve information about specific bills.
- **Law**: Access details about enacted laws.
- **Amendment**: Get information on amendments.
- **Summaries**: Obtain summaries of bills.
- **Congress**: Fetch details about a specific Congress session.
- **Member**: Information about members of Congress.
- **Committee**: Details on committees and their activities.
- **Hearing**: Information on hearings.
- **CongressionalRecord**: Access the Congressional Record.
- **Treaty**: Details about treaties.

Refer to the [US Congress API Documentation](https://api.congress.gov/) for detailed information on each endpoint.

## Common Parameters

Most endpoints support a set of common query parameters encapsulated in the `CommonParams` struct:

- **format**: `Json` or `Xml`
- **offset**: Pagination offset
- **limit**: Number of records to retrieve
- **from_date_time**: Start date-time in `YYYY-MM-DDTHH:MM:SSZ` format
- **to_date_time**: End date-time in `YYYY-MM-DDTHH:MM:SSZ` format
- **sort**: Sorting order (`DateAsc`, `DateDesc`)

Example:

```rust
let params = CommonParams {
    format: Some(cdg_api::Format::Json),
    limit: Some(20),
    sort: Some(cdg_api::Sort::DateAsc),
    ..CommonParams::new()
};
```
## Future Work

- Add support for additional parameters options.
- Better parameter validation and error handling.
- Response parsing into Rust data structures.
- Higher-level abstractions for common use cases.

## License

This project is licensed under the [MIT License](LICENSE).
