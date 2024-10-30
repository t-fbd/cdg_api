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
  - **`endpoints`**: Contains enums and functions representing the available API endpoints, allowing you to fetch data about bills, members, and more.
  - **`url_builders`**: Utility functions for constructing proper API URLs with query parameters.
  - **`param_models`**: Defines models and enums for different query parameters, facilitating type-safe request building.
  - **`response_models`**: Data structures representing the API's response data, enabling easy deserialization and manipulation of returned JSON data.
  - **`client`**: Contains the `CongressApiClient` struct and its associated methods for interacting with the API.
  - **`request_handlers`**: Provides functions for making HTTP requests and handling responses, with support for both `reqwest` and `curl+jq` request methods.

- **Convenient Request Utilities**:
  - **`CongressApiClient`**: A centralized client for interacting with the US Congress API, managing API keys, constructing URLs, making requests, and deserializing responses.

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

Before using the library, you need to obtain an API key from the [US Congress API](https://api.congress.gov/). Once you have your API key, you can provide it to the `CongressApiClient` in one of two ways:

1. **Environment Variable**: Set the `CDG_API_KEY` environment variable with your API key.

   ```bash
   export CDG_API_KEY="your_api_key_here"
   ```

2. **Direct Initialization**: Pass the API key directly when creating a new instance of `CongressApiClient`.

   ```rust
   use cdg_api::CongressApiClient;

   let client = CongressApiClient::new(Some("your_api_key_here".to_string())).unwrap();
   ```

**Note**: Setting the API key via environment variables ensures that you don't have to hardcode sensitive information in your code.

## Usage

### Using `CongressApiClient`

The `CongressApiClient` provides a streamlined way to interact with various API endpoints. Below are examples of how to use the client to fetch different types of data, leveraging the `NewEndpoint` trait and enums for parameter values to ensure type safety.

#### Example 1: Fetching Members

Here's an example of how to fetch a list of current members of Congress and display their names, party affiliations, and states.

```rust

use cdg_api::CongressApiClient;
use cdg_api::endpoints::{Endpoints, NewEndpoint};
use cdg_api::param_models::{FormatType, MemberListParams};
use cdg_api::response_models::MembersResponse;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let client = CongressApiClient::new(None)?; // Use default API key from environment
                                               // Set the API key explicitly with:
                                               // CongressApiClient::new(Some("api_key_here".to_string()))?;
    // Define parameters using enums
    let params = MemberListParams {
        format: Some(FormatType::Json),
        limit: Some(10),
        current_member: Some(true),
        ..MemberListParams::default()
    };

    // Create the endpoint using the NewEndpoint trait
    let endpoint = Endpoints::new_member_list(params);

    // Fetch the data, casting it to the appropriate response type
    let response: MembersResponse = client.fetch(endpoint)?;

    // Process the response
    for member in response.members {
        println!("{}, {}, {}\n", member.name, member.state, member.party_name);
    }

    Ok(())
}
```

#### Example 2: Fetching Bills

Here's how to fetch a list of bills and display their titles, types, and numbers, utilizing the `BillType` enum.

```rust
use cdg_api::CongressApiClient;
use cdg_api::endpoints::{Endpoints, NewEndpoint};
use cdg_api::param_models::{FormatType, BillByTypeParams, BillType};
use cdg_api::response_models::BillsResponse;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = CongressApiClient::new(None)?;

    // Define parameters using enums
    let params = BillByTypeParams {
        format: Some(FormatType::Json),
        limit: Some(10),
        ..BillByTypeParams::default()
    };

    // Create the endpoint using the NewEndpoint trait
    let endpoint = Endpoints::new_bill_by_type(118, BillType::Hr, params);

    // Fetch the data, casting it to the appropriate response type
    let response: BillsResponse = client.fetch(endpoint)?;

    // Process the response
    for bill in response.bills {
        println!("{}, {}, {}\n", bill.title, bill.bill_type, bill.number);
    }

    Ok(())
}
```

## Modules

- **`endpoints`**: Contains enums and functions representing the available API endpoints. It provides constructors (via the `NewEndpoint` trait) for each endpoint variant, allowing you to specify which endpoint you want to interact with and define the necessary parameters.

- **`url_builders`**: Provides helper functions, such as `generate_url`, for constructing complete API URLs with necessary query parameters.

- **`param_models`**: Includes data models and enums for API query parameters, ensuring type-safe and flexible request customization. Enums like `FormatType`, `SortType`, `BillType`, `AmendmentType`, `ChamberType`, `CommunicationType`, and `LawType` are defined here to represent specific parameter values.

- **`response_models`**: Defines the data structures corresponding to API responses, leveraging `serde` for seamless JSON deserialization.

- **`client`**: Contains the `CongressApiClient` struct and its associated methods for interacting with the API.

- **`request_handlers`**: Provides functions for making HTTP requests and handling responses. Reqwest or curl+jq entry points are available for making requests. Typically, the `reqwest` is the better choice due to ease of use with the response models.


## Error Handling

The library defines a custom error type `ApiClientError` to handle various error scenarios, including HTTP errors, URL construction issues, deserialization failures, and environment variable access problems. The `fetch` method returns `Result<T, ApiClientError>`, allowing for more granular and precise error handling.

### Error Variants

- **`ApiClientError::Http`**: Represents HTTP-related errors, including network issues and non-success status codes.
- **`ApiClientError::Deserialization`**: Occurs when there is an error deserializing the API response into the desired Rust struct.
- **`ApiClientError::Url`**: Triggered when there is an error constructing the API URL.
- **`ApiClientError::EnvVar`**: Happens when the API key is not found in the environment variables.

## License

This project is licensed under the terms of the [MIT license](https://github.com/t-fbd/cdg_api/blob/master/LICENSE).

## Repository

[https://github.com/t-fbd/cdg_api](https://github.com/t-fbd/cdg_api)
