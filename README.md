# cdg_api

A simple Rust library to interact with the [US Congress API](https://api.congress.gov/).

![Crates.io](https://img.shields.io/crates/v/cdg_api)
![MIT License](https://img.shields.io/crates/l/cdg_api)

## Overview

`cdg_api` provides a Rust interface for interacting with the US Congress API. It simplifies the process of constructing API endpoints, building URLs with parameters, and retrieving legislative data. Whether you're fetching information about bills, members, amendments, or laws, `cdg_api` offers a streamlined and type-safe approach to accessing this data.

### Currently Unavailable Endpoints

The following API endpoints are currently unavailable for making calls, although response objects may be available for most of them:

- `/committee-report`
- `/committee-print`
- `/committee-meeting`
- `/hearing`
- `/congressional-record`
- `/daily-congressional-record`
- `/bound-congressional-record`
- `/house-communication`
- `/house-requirement`
- `/senate-communication`

### Available Endpoints

The library currently supports endpoints related to:

- `/member`
- `/bill`
- `/law`
- `/amendment`
- `/committee`
- `/nomination`
- `/treaty`
- `/summaries`
- `/congress`

## Features

- **Modular Design**: Organized into distinct modules to separate concerns, enhancing maintainability and scalability.
  - **`endpoints`**: Enums and functions representing the available API endpoints.
  - **`url_builders`**: Utility functions for constructing API URLs with query parameters.
  - **`param_models`**: Models and enums for different query parameters.
  - **`response_models`**: Data structures for API responses, including specific models and the versatile `GenericResponse`.
  - **`client`**: `CongressApiClient` struct for interacting with the API.
  - **`request_handlers`**: Functions for making HTTP requests and handling responses.

- **Convenient Request Utilities**:
  - **`CongressApiClient`**: Centralized client managing API keys, constructing URLs, making requests, and deserializing responses.

- **Flexible Response Handling**:
  - **`GenericResponse`**: A catch-all response model for endpoints without a specific response type or when the response model is unknown.

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

`CongressApiClient` allows you to interact with various API endpoints. Below are examples demonstrating how to fetch different types of data.

#### Example 1: Fetching Members

Fetch a list of current members of Congress and display their names, party affiliations, and states.

```rust
use cdg_api::CongressApiClient;
use cdg_api::endpoints::{Endpoints, NewEndpoint};
use cdg_api::param_models::{FormatType, MemberListParams};
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
use cdg_api::param_models::{FormatType, BillByTypeParams, BillType};
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

### Using `GenericResponse`

When a specific response model is not defined or the response structure is unknown, use `GenericResponse` to handle the response dynamically.

#### What is `GenericResponse`?

`GenericResponse` is an enum-based catch-all response model that can represent any of the defined response types. It leverages the `GenericResponseModel` enum to encapsulate various possible responses, making it flexible for handling diverse API responses.

#### Working Example

```rust
use cdg_api::CongressApiClient;
use cdg_api::endpoints::Endpoints;
use cdg_api::param_models::{FormatType, BillListParams};
use cdg_api::response_models::GenericResponse;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let client = CongressApiClient::new(None)?; // Use default API key

    // Define parameters
    let params = BillListParams {
        format: Some(FormatType::Json),
        limit: Some(10),
        ..BillListParams::default()
    };

    // Create the endpoint
    let endpoint = Endpoints::BillList(params);

    // Fetch the data
    let response: GenericResponse = client.fetch(endpoint)?;

    // Process the response
    println!("{}", response.serialize_generic_response(true)?);

    Ok(())
}
```

#### Advantages of Using `GenericResponse`

- **Flexibility**: Handle any response structure without needing a specific model.
- **Simplicity**: Reduce the need to manage numerous specific response models.
- **Future-Proofing**: Accommodate new or unexpected response types without immediate library updates.

#### Considerations

- **Type Safety**: While flexible, it requires handling enum variants to access specific data.
- **Complexity**: Managing a large enum with many variants can be cumbersome.
- **Performance**: Potential overhead due to extensive enum matching; ensure it meets your performance needs.

## Modules

- **`endpoints`**: Enums and functions representing the available API endpoints.
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

## License

This project is licensed under the terms of the [MIT license](https://github.com/t-fbd/cdg_api/blob/master/LICENSE).

## Repository

[https://github.com/t-fbd/cdg_api](https://github.com/t-fbd/cdg_api)
