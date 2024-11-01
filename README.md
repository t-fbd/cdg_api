# cdg_api

A simple Rust library to interact with the [US Congress API](https://api.congress.gov/).

![Crates.io](https://img.shields.io/crates/v/cdg_api)
![MIT License](https://img.shields.io/crates/l/cdg_api)

`cdg_api` provides a Rust interface for interacting with the US Congress API. It simplifies constructing API endpoints, building URLs with parameters, and retrieving legislative data. Whether fetching information about bills, members, amendments, or laws, `cdg_api` offers a streamlined and type-safe approach to accessing this data.

The library is a work in progress but is very much functional. It is designed to be modular, extensible, and easy to use. The [`CongressApiClient`] struct serves as the central component for making requests, handling responses, and managing API keys. The library includes specific models for various API responses, such as bills, members, nominations, and treaties, as well as a versatile `GenericResponse` for handling unknown response structures.

Please, if you find any issues, particularly with the API responses or models as I find it tedious to test all of them, feel free to open an issue or submit a pull request.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Installation](#installation)
- [Getting Started](#getting-started)
  - [Setting Up Your API Key](#setting-up-your-api-key)
- [Using `CongressApiClient`](#using-congressapiclient)
  - [Example 1: Fetching Members](#example-1-fetching-members)
  - [Example 2: Using `GenericResponse` with `parse_generic_response`](#example-2-using-genericresponse-with-parse_generic_response)
  - [Example 3: Using `Endpoints::Generic` for Custom Endpoints](#example-3-using-endpointsgeneric-for-custom-endpoints)
- [Other Projects](#other-projects)
- [License](#license)
- [Repository](#repository)

## Quick Info

Around **100+** endpoints models available for interacting with the US Congress API, as well as the ability to create custom endpoints using `Endpoints::Manual`.

Around **150+** response models available for parsing API responses, including specific models for bills, members, nominations, treaties, and more.

- **Modules**:
    - **[`endpoints`]**: Models representing available API endpoints, including `Endpoints::Generic` for custom endpoints.
    - **[`url_builders`]**: Utility functions for constructing API URLs with query parameters.
    - **[`param_models`]**: Models and enums for different query parameters.
    - **[`response_models`]**: Models for API responses, including specific models and the versatile `GenericResponse`.
    - **[`cdg_client`]**: [`CongressApiClient`] struct for interacting with the API.
    - **[`cdg_types`]**: Enums and structs and implementations for various custom types used in the API.

- **Api Client**:
  - **[`CongressApiClient`]**: Centralized client managing API keys, constructing URLs, making requests, and deserializing responses.

- **Generic Response Handling**:
  - **`GenericResponse`**: A catch-all response model for endpoints without a specific response type or when the response model is unknown.
  - **[`parse_generic_response`]**: A method to parse `GenericResponse` into a specific response model when the structure is known.
  - **[`serialize_generic_response`]**: A method to serialize `GenericResponse` into a JSON string for debugging or creating a specific response model, a good fallback when parsing fails.

- **Modules by Feature Flags**:
  - **Feature Flag: `requests` (enabled by default)**:
    - **[`request_handlers`]**: Functions for making HTTP requests and handling responses, parts of which are used by [`CongressApiClient`].

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

If you don't want to pull in reqwest as a dependency, you can disable the `requests` feature by just disabling the default features:

```toml
[dependencies]
cdg_api = { version = "*", default-features = false }
```

or

```bash
cargo add cdg_api --no-default-features
```

## Getting Started

### Setting Up Your API Key

Obtain an API key from the [US Congress API](https://api.congress.gov/). Provide it to the [`CongressApiClient`] either via an environment variable or direct initialization:

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

## Using [`CongressApiClient`]

[`CongressApiClient`] allows you to interact with various API endpoints. Below are examples demonstrating how to fetch different types of data, including the new `Endpoints::Manual` variant.

#### Example 1: Fetching Members

Fetch a list of current members of Congress and display their names, party affiliations, and states.

ps. Instead of using the standard [`unwrap`] methods, you can use [`cdg_api::unwrap_option()`], [`cdg_api::unwrap_option_string()`], or [`cdg_api::unwrap_option_u32()`]
    in order to quickly unwrap the [`Option`] values and provide a default value if the value is [`None`]. Pretty much the same as [`unwrap_or_default()`].

```rust
use cdg_api::CongressApiClient;
use cdg_api::endpoints::{Endpoints, NewEndpoint};
use cdg_api::param_models::MemberListParams;
use cdg_api::cdg_types::FormatType;
use cdg_api::response_models::MembersResponse;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = CongressApiClient::new(None)?; // Use API key from environment

    // Create the endpoint and define parameters
    let endpoint = Endpoints::new_member_list(
        MemberListParams {
            format: Some(FormatType::Json),
            limit: Some(10),
            current_member: Some(true),
            ..MemberListParams::default()
        }
    );

    // Fetch the data
    let response: MembersResponse = client.fetch(endpoint)?;

    // Process the response
    for member in response.members {
        println!("{}, {}, {}\n", 
            member.name.unwrap_or_default(),
            member.state.unwrap_or_default(),
            member.party_name.unwrap_or_default()
        );
    }

    Ok(())
}
```

#### Example 2: Using `GenericResponse` with `parse_generic_response`

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
    let bill_details: BillDetailsResponse = match response.parse_generic_response() {
        Ok(bill_details) => bill_details,
        Err(e) => {
            // If parsing fails to a specific primary response fails, the GenericResponse can be serialized
            // to see the response structure. This can help in debugging and creating a specific response model.
            // The argument `true` pretty prints the response, `false` prints the raw JSON.
            println!("Failed to parse response: {}", e);
            println!("Response:\n\n{}", response.serialize_generic_response(true)?);
            return Ok(());
        }
    };
    let bill = bill_details.bill;

    println!("Bill: {}", bill.number.unwrap_or_default());
    println!("Title: {}", bill.title.unwrap_or_default());
    println!("Summary: {:#?}", bill.summaries.unwrap_or_default());

    Ok(())
}
```

#### Example 3: Using `Endpoints::Generic` for Custom Endpoints

When working with custom or unknown endpoints, you can use `Endpoints::Generic` to specify the endpoint string such as `daily-congressional-record` and `GenericParams` to define query parameters. The response can be fetched as `GenericResponse`.
The `Endpoint` created can then call `parse_generic_response` to parse the response into a specific response model. If parsing fails, the `GenericResponse` can be serialized to see the response structure.

This is essentially the Endpoints equivalent of the GenericResponse example above. One for requests and the other for responses.

```rust
use cdg_api::CongressApiClient;
use cdg_api::endpoints::{Endpoints, NewEndpoint};
use cdg_api::response_models::{DailyCongressionalRecordResponse, GenericResponse};
use cdg_api::param_models::GenericParams;
use cdg_api::cdg_types::*;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let client = CongressApiClient::new(None)?; // Use API key from environment

    // Manually specify the endpoint string
    let endpoint = Endpoints::new_generic(
      "daily-congressional-record".to_string(),
      GenericParams::new(
        FormatType::Json.into(),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
      )
    );

    // Fetch the data as GenericResponse
    let response: GenericResponse = client.fetch(endpoint)?;

    // Parse the response into DailyCongressionalRecordResponse
    match response.parse_generic_response::<DailyCongressionalRecordResponse>() {
        Ok(daily_record) => {
            let record = daily_record.daily_congressional_record;
            for issue in record {
                println!("Issue Number: {}", issue.issue_number.unwrap_or_default());
                println!("Volume Number: {}", issue.volume_number.unwrap_or_default());
                println!("Issue Date: {}", issue.issue_date.unwrap_or_default());
                println!("Congress: {}", issue.congress.unwrap_or_default());
                println!("Session Number: {}", issue.session_number.unwrap_or_default());
                println!("URL: {}", issue.url.unwrap_or_default());
                println!("Sections:");
                if let Some(full) = issue.full_issue {
                    println!("Full Record: {:#?}", full);
                }
                println!("----------------------------");
            }
        },
        Err(e) => {
            println!("Failed to parse response: {}", e);
            println!("Response:\n\n{}", response.serialize_generic_response(true)?);
        }
    }

    Ok(())
}
```

## Other Projects

- **[`loc_api`](https://crates.io/crates/loc_api)**: A Rust library for interacting with the Library of Congress API.

## License

This project is licensed under the terms of the [MIT license](https://github.com/t-fbd/cdg_api/blob/master/LICENSE).

## Repository

[https://github.com/t-fbd/cdg_api](https://github.com/t-fbd/cdg_api)


