# cdg_api

A simple Rust library to interact with the [US Congress API](https://api.congress.gov/).

![Crates.io](https://img.shields.io/crates/v/cdg_api)
![MIT License](https://img.shields.io/crates/l/cdg_api)

## Overview

`cdg_api` provides a Rust interface for interacting with the US Congress API. It simplifies constructing API endpoints, building URLs with parameters, and retrieving legislative data. Whether fetching information about bills, members, amendments, or laws, `cdg_api` offers a streamlined and type-safe approach to accessing this data.

## Features

- **Modular Design**: Organized into distinct modules for maintainability and scalability.
  - **`endpoints`**: Enums and functions representing available API endpoints, including `Endpoints::Manual` for custom endpoints.
  - **`url_builders`**: Utility functions for constructing API URLs with query parameters.
  - **`param_models`**: Models and enums for different query parameters.
  - **`response_models`**: Data structures for API responses, including specific models and the versatile `GenericResponse`.
  - **`client`**: `CongressApiClient` struct for interacting with the API.
  - **`request_handlers`**: Functions for making HTTP requests and handling responses.

- **Convenient Request Utilities**:
  - **`CongressApiClient`**: Centralized client managing API keys, constructing URLs, making requests, and deserializing responses.

- **Flexible Response Handling**:
  - **`GenericResponse`**: A catch-all response model for endpoints without a specific response type or when the response model is unknown.
  - **`parse_generic_response`**: A method to parse `GenericResponse` into a specific response model when the structure is known.

## Installation

Add `cdg_api` to your `Cargo.toml`:

```toml
[dependencies]
cdg_api = "*"
```

Or use `cargo` to add the dependency:

```bash
cargo add cdg_api
```

## Getting Started

### Setting Up Your API Key

Obtain an API key from the [US Congress API](https://api.congress.gov/). Provide it to the `CongressApiClient` either via an environment variable or direct initialization:

1. **Environment Variable**:

   ```bash
   export CDG_API_KEY="your_api_key_here"
   ```

2. **Direct Initialization**:

   ```rust
   use cdg_api::CongressApiClient;

   let client = CongressApiClient::new(Some("your_api_key_here".to_string())).unwrap();
   ```

**Note**: Using environment variables is recommended to avoid hardcoding sensitive information.

## Usage

### Using `CongressApiClient`

`CongressApiClient` allows you to interact with various API endpoints. Below are examples demonstrating how to fetch different types of data, including the new `Endpoints::Manual` variant.

#### Example 1: Fetching Members

Fetch a list of current members of Congress and display their names, party affiliations, and states.

```rust
use cdg_api::CongressApiClient;
use cdg_api::endpoints::{Endpoints, NewEndpoint};
use cdg_api::param_models::MemberListParams;
use cdg_api::cdg_types::FormatType;
use cdg_api::response_models::MembersResponse;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = CongressApiClient::new(None)?; // Use API key from environment

    // Define parameters
    let params = MemberListParams {
        format: Some(FormatType::Json),
        limit: Some(10),
        current_member: Some(true),
        ..MemberListParams::default()
    };

    // Create the endpoint
    let endpoint = Endpoints::new_member_list(params);

    // Fetch the data
    let response: MembersResponse = client.fetch(endpoint)?;

    // Process the response
    for member in response.members {
        println!("{}, {}, {}\n", member.name, member.state, member.party_name);
    }

    Ok(())
}
```

#### Example 2: Fetching Bills

Fetch a list of bills and display their titles, types, and numbers.

```rust
use cdg_api::CongressApiClient;
use cdg_api::endpoints::{Endpoints, NewEndpoint};
use cdg_api::param_models::BillByTypeParams;
use cdg_api::cdg_types::{BillType, FormatType};
use cdg_api::response_models::BillsResponse;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = CongressApiClient::new(None)?;

    // Define parameters
    let params = BillByTypeParams {
        format: Some(FormatType::Json),
        limit: Some(10),
        ..BillByTypeParams::default()
    };

    // Create the endpoint
    let endpoint = Endpoints::new_bill_by_type(118, BillType::Hr, params);

    // Fetch the data
    let response: BillsResponse = client.fetch(endpoint)?;

    // Process the response
    for bill in response.bills {
        println!("{}, {}, {}\n", bill.title, bill.bill_type, bill.number);
    }

    Ok(())
}
```

#### Example 3: Using `GenericResponse` with `parse_generic_response`

Fetch detailed information about a specific bill using `GenericResponse` and parse it into a specific response model.

```rust
use cdg_api::CongressApiClient;
use cdg_api::endpoints::{Endpoints, NewEndpoint};
use cdg_api::param_models::BillDetailsParams;
use cdg_api::cdg_types::{BillType, FormatType};
use cdg_api::response_models::{BillDetailsResponse, GenericResponse};

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let client = CongressApiClient::new(None)?; // Use API key from environment

    // Define parameters for bill details
    let params = BillDetailsParams {
        format: Some(FormatType::Json),
        ..BillDetailsParams::default()
    };

    // Specify the bill to fetch (e.g., H.R. 1234 from the 118th Congress)
    let endpoint = Endpoints::new_bill_details(118, BillType::Hr, 148, params);

    // Fetch the data as GenericResponse
    let response: GenericResponse = client.fetch(endpoint)?;

    // Parse the response as BillDetailsResponse
    let bill_details: BillDetailsResponse = response.parse_generic_response()?;
    let bill = bill_details.bill;

    println!("Bill: {}", bill.number);
    println!("Title: {}", bill.title);
    println!("Summary: {:#?}", bill.summaries);

    Ok(())
}
```

#### Example 4: Using `Endpoints::Manual` for Custom Endpoints

Fetch data from a manually specified endpoint string using `Endpoints::Manual`.

```rust
use cdg_api::CongressApiClient;
use cdg_api::endpoints::{Endpoints, NewEndpoint};
use cdg_api::response_models::{DailyCongressionalRecordResponse, GenericResponse};

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let client = CongressApiClient::new(None)?; // Use API key from environment

    // Manually specify the endpoint string
    let endpoint = Endpoints::new_manual("/daily-congressional-record?format=json".to_string());

    // Fetch the data as GenericResponse
    let response: GenericResponse = client.fetch(endpoint)?;

    // Parse the response into DailyCongressionalRecordResponse
    match response.parse_generic_response::<DailyCongressionalRecordResponse>() {
        Ok(daily_record) => {
            let record = daily_record.daily_congressional_record;
            for issue in record {
                println!("Issue Number: {}", issue.issue_number);
                println!("Volume Number: {}", issue.volume_number);
                println!("Issue Date: {}", issue.issue_date);
                println!("Congress: {}", issue.congress);
                println!("Session Number: {}", issue.session_number);
                println!("URL: {}", issue.url);
                println!("Sections:");
                if let Some(full) = issue.full_issue {
                    println!("Full Record: {:#?}", full);
                }
                println!("----------------------------");
            }
        },
        Err(e) => {
            println!("Failed to parse response: {}", e);
        }
    }

    Ok(())
}
```

### Using `GenericResponse` for Manual Endpoints

When using `Endpoints::Manual`, `GenericResponse` allows you to handle responses dynamically without predefined response models. You can parse the response into any specific model if you know its structure, enhancing flexibility.

## Modules

- **`endpoints`**: Enums and functions representing available API endpoints, including `Endpoints::Manual` for custom endpoint strings.
- **`url_builders`**: Helper functions for constructing complete API URLs with query parameters.
- **`param_models`**: Data models and enums for API query parameters, ensuring type-safe requests.
- **`response_models`**: Data structures for API responses, including specific models and `GenericResponse`.
- **`client`**: `CongressApiClient` struct and methods for interacting with the API.
- **`request_handlers`**: Functions for making HTTP requests and handling responses, supporting `reqwest` and `curl+jq`.

## Error Handling

The library defines a custom error type `ApiClientError` to handle various error scenarios, including HTTP errors, URL construction issues, deserialization failures, and environment variable access problems. The `fetch` method returns `Result<T, ApiClientError>`, allowing for precise error handling.

### Error Variants

- **`ApiClientError::Http`**: HTTP-related errors, including network issues and non-success status codes.
- **`ApiClientError::Deserialization`**: Errors during deserializing the API response.
- **`ApiClientError::Url`**: Errors constructing the API URL.
- **`ApiClientError::EnvVar`**: Missing API key in environment variables.

## Command-Line Interface (`main.rs`)

The `main.rs` file serves as a simple CLI for interacting with the `cdg_api` library. It allows users to execute various commands to fetch and display data from the US Congress API in a user-friendly format.

### Available Commands

- **`list_bills`**  
  *List recent bills introduced in Congress.*

- **`current_congress`**  
  *Display information about the current congress session.*

- **`list_nominations`**  
  *List recent nominations.*

- **`list_treaties`**  
  *List recent treaties.*

- **`member_details <bioguide_id>`**  
  *Get detailed information about a specific member using their `bioguide_id`.*

- **`bill_details <congress> <bill_type> <bill_number>`**  
  *Get detailed information about a specific bill by specifying the congress number, bill type (see `BillType`), and bill number.*

### Usage

1. **Set the API Key**  
   Ensure that the `CDG_API_KEY` environment variable is set with your Congress API key:
   ```bash
   export CDG_API_KEY="your_api_key_here"
   ```

2. Run the Application


Use the following command structure to execute the application with the desired command:

```bash
cargo run -- <command> [additional arguments]
```

Examples

List Recent Bills
```bash
cargo run -- list_bills
```

Get Member Details
```bash
cargo run -- member_details A000360
```

Get Bill Details
```bash
cargo run -- bill_details 117 H 1150
```

## Other Projects

- **[`loc_api`](https://crates.io/crates/loc_api)**: A Rust library for interacting with the Library of Congress API.

## License

This project is licensed under the terms of the [MIT license](https://github.com/t-fbd/cdg_api/blob/master/LICENSE).

## Repository

[https://github.com/t-fbd/cdg_api](https://github.com/t-fbd/cdg_api)
