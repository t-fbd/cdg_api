# cdg_api

A simple library to interact with the [US Congress API](https://api.congress.gov/).

![Crates.io](https://img.shields.io/crates/v/cdg_api)
![MIT License](https://img.shields.io/crates/l/cdg_api)

## Overview

This library provides a Rust interface for the US Congress API, allowing you to construct API endpoints, build URLs with parameters, and retrieve legislative data.

Currently unavailable in the API are the following:
calls to the endpoints:

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

Available endpoints are all related to:

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

- **Enumerated Endpoints**: Covering various legislative data.
- **Parameter Models**: For customizing API queries in a type-safe manner.
- **URL Builders**: For constructing API URLs with parameters.
- **Response Models**: Structs representing the API responses, enabling easy deserialization and manipulation of returned JSON data.
- **Utility Functions**:
  - **`curl_and_jq`**: Fetch and pretty-print JSON responses (requires `curl` and `jq`).
  - **`get_congress_data`**: Fetch data using `reqwest` and deserialize into Rust structs.

## Installation

Add `cdg_api` to your `Cargo.toml`:

```toml
[dependencies]
cdg_api = "0.2.0"
```

or use `cargo` to add the dependency:

```bash
cargo add cdg_api
```

## Getting Started

### Setting Up Your API Key

Before using the library, you need to obtain an API key from the [US Congress API](https://api.congress.gov/).

Set the `CDG_API_KEY` environment variable in your system:

- **Unix/Linux/macOS**:

  ```bash
  export CDG_API_KEY=your_api_key_here
  ```

- **Windows (Command Prompt)**:

  ```cmd
  set CDG_API_KEY=your_api_key_here
  ```

- **Windows (PowerShell)**:

  ```powershell
  $env:CDG_API_KEY="your_api_key_here"
  ```

Replace `your_api_key_here` with your actual API key.

**Note**: Setting the API key via environment variables ensures that you don't have to hardcode sensitive information in your code.

## Usage

### Basic Example

Here's an example of how to fetch a list of current members of Congress and display their names, party affiliations, and states.

**Note**:

- This example uses the `get_congress_data` function, which requires the `reqwest` crate (already included as a dependency).
- Ensure that the `CDG_API_KEY` environment variable is set as described above.

```rust
use std::process::exit;

use cdg_api::{
    url_builders::get_endpoint_url,
    Endpoints, NewEndpoint, 
    param_models::{
        FormatType, MemberListParams
    },
    response_models::MembersResponse,
    get_congress_data
};

fn main() {
    let params = MemberListParams {
        format: Some(FormatType::Json),
        limit: Some(10),
        current_member: Some(true),
        ..MemberListParams::default()
    };

    let endpoint = Endpoints::new_member_list(params);

    let url = get_endpoint_url(endpoint);

    println!("URL: {}", url);

    let response: MembersResponse = match get_congress_data(&url) {
        Ok(response) => response,
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(1);
        }
    };

    for member in response.members {
        println!("{}, {}, {}\n", member.name, member.state, member.party_name);
    }
}
```

This example:

- Constructs an endpoint for fetching a list of current members of Congress.
- Builds the full API URL using the `get_endpoint_url` function.
- Uses the `get_congress_data` function to fetch and deserialize the data into `MembersResponse`.
- Iterates through the members and prints their names, states, and party affiliations.

### Using `curl_and_jq`

Alternatively, you can use the `curl_and_jq` utility function to fetch and process JSON data directly in the command line.

```rust
use std::process::exit;

use cdg_api::{
    url_builders::get_endpoint_url,
    Endpoints, NewEndpoint, 
    param_models::{
        FormatType, MemberListParams
    },
    response_models::*,
    curl_and_jq
};

fn main() {
    let params = MemberListParams {
        format: Some(FormatType::Json),
        limit: Some(10),
        current_member: Some(true),
        ..MemberListParams::default()
    };

    let endpoint = Endpoints::new_member_list(params);

    let url = get_endpoint_url(endpoint);

    println!("URL: {}", url);

    match curl_and_jq(
        &url,
        ".members | to_entries[] | {name: .value.name, party: .value.partyName, state: .value.state}",
    ) {
        Ok(_) => exit(0),
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(1);
        }
    }
}
```

This example:

- Constructs an endpoint for fetching a list of current members of Congress.
- Builds the full API URL using the `get_endpoint_url` function.
- Uses the `curl_and_jq` utility function to fetch the data and process it with `jq` to display selected fields.

### Requirements

- **curl**: Command-line tool for transferring data with URLs.
- **jq**: Lightweight and flexible command-line JSON processor.

You can install them using:

- **Ubuntu/Debian**:

  ```bash
  sudo apt-get install curl jq
  ```

- **macOS (Homebrew)**:

  ```bash
  brew install curl jq
  ```

- **Windows**:

  - [Download curl](https://curl.se/windows/)
  - [Download jq](https://stedolan.github.io/jq/download/)

## Modules

### `endpoints`

Contains the `Endpoints` enum and the `NewEndpoint` trait, representing available API endpoints.

#### `Endpoints` Enum

Variants for various data types:

- **Members**: `MemberList`, `MemberByCongress`, `MemberByState`, etc.
- **Bills**, **Laws**, **Amendments**, **Committees**, **Nominations**, **Treaties**, **Summaries**, **Congress**.

#### `NewEndpoint` Trait

Provides constructors for each endpoint variant.

```rust
use cdg_api::{Endpoints, NewEndpoint};
use cdg_api::param_models::MemberListParams;

let params = MemberListParams {
    current_member: Some(true),
    limit: Some(10),
    ..Default::default()
};

let endpoint = Endpoints::new_member_list(params);
```

### `url_builders`

Utilities to construct API URLs with parameters.

- **`get_endpoint_url`**: Generates the full API URL by combining the base URL, endpoint path, query parameters, and API key.

### `param_models`

Data models for API parameters and enumerations used to customize API queries.

#### Enums

- **FormatType**: Response format (`Json`, `Xml`).
- **SortType**: Sorting options (`UpdateDateAsc`, `UpdateDateDesc`).
- **BillType**: Bill categories (`Hr`, `S`, etc.).
- **AmendmentType**: Amendment categories (`Hamdt`, `Samdt`, `Suamdt`).
- **ChamberType**: Legislative chambers (`House`, `Senate`, `Joint`).
- **CommunicationType**: Committee communications (`Ec`, `Ml`, `Pm`, `Pt`).
- **LawType**: Law types (`Pub`, `Priv`).

#### Structs

Parameter structs for customizing API requests, such as:

- **Member Parameters**
  - `MemberListParams`
  - `MemberByStateParams`
  - `MemberByCongressParams`
  - `MemberByCongressStateDistrictParams`
  - `MemberDetailsParams`
  - `SponsorshipListParams`
  - `CosponsorshipListParams`

*(Other parameter structs for bills, laws, amendments, etc., are also available.)*

### `response_models`

Defines the data structures that map to the API response JSON structures using `serde` for deserialization.

#### PrimaryResponse Trait

A trait that all primary response structs implement, ensuring they can be deserialized and used with `get_congress_data`.

#### Structs

- **MembersResponse**: Represents the response structure for member-related endpoints.
- **BillsResponse**: Represents the response structure for bill-related endpoints.
- **LawsResponse**: Represents the response structure for law-related endpoints.
- **AmendmentsResponse**: Represents the response structure for amendment-related endpoints.
- *(Additional response structs for committees, nominations, treaties, summaries, congress, etc.)*

#### Example Response Struct

```rust
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MembersResponse {
    pub members: Vec<Member>,
    // Additional fields as per the API response
}

#[derive(Debug, Deserialize)]
pub struct Member {
    pub name: String,
    pub state: String,
    pub party_name: String,
    // Additional member fields
}
```

### `param_models`

*(This section remains unchanged as it's already well-documented.)*

## Functions

### `get_congress_data`

Fetches data from the US Congress API and deserializes it into the specified response model.

```rust
pub fn get_congress_data<T: PrimaryResponse + DeserializeOwned>(url: &str) -> Result<T, Box<dyn std::error::Error>>;
```

- **Parameters**:
  - `url`: The API endpoint URL.
  
- **Returns**:
  - `Ok(T)`: The deserialized response data.
  - `Err`: An error if the request fails or deserialization fails.

**Usage Example**:

```rust
use cdg_api::{
    get_congress_data,
    response_models::MembersResponse,
};

fn fetch_members(url: &str) -> Result<MembersResponse, Box<dyn std::error::Error>> {
    let response: MembersResponse = get_congress_data(url)?;
    Ok(response)
}
```

### `curl_and_jq`

Utility function that performs a `curl` request and processes the JSON output with `jq`.

```rust
pub fn curl_and_jq(url: &str, jq_cmd: &str) -> Result<(), Box<dyn std::error::Error>>;
```

- **Parameters**:
  - `url`: The API endpoint URL.
  - `jq_cmd`: A `jq` filter string to process the JSON output.

**Note**:

- Requires `curl` and `jq` to be installed.
- Ensures that the `CDG_API_KEY` environment variable is set.

**Usage Example**:

```rust
use cdg_api::{
    curl_and_jq, url_builders::get_endpoint_url,
    Endpoints, NewEndpoint, param_models::MemberListParams,
};

fn main() {
    // Ensure CDG_API_KEY is set in your environment

    let params = MemberListParams {
        limit: Some(5),
        ..Default::default()
    };

    let endpoint = Endpoints::new_member_list(params);
    let url = get_endpoint_url(endpoint);

    curl_and_jq(&url, ".members | to_entries[] | {name: .value.name, state: .value.state}").unwrap();
}
```

## Environment Variables

- **`CDG_API_KEY`**: Your API key for the US Congress API.

## Future Work

Primary areas for future development include:

- **Async Support**: Introduce asynchronous HTTP request support using `tokio` or `async-std` for better performance in applications that can leverage it.
- **Enhanced Error Handling**: Improve error handling mechanisms to provide more informative and user-friendly feedback.
- **Additional Response Models**: Expand the `response_models` to cover all available API endpoints comprehensively.
- **Pagination Support**: Implement support for handling paginated responses from the API.
- **Caching Mechanisms**: Introduce caching to reduce redundant API calls and improve performance.
- **Comprehensive Documentation**: Expand the documentation with more examples and detailed explanations of each module and function.

Feel free to open an issue or submit a pull request on the [GitHub repository](https://github.com/t-fbd/cdg_api).

## License

This project is licensed under the terms of the [MIT license](https://github.com/t-fbd/cdg_api/blob/master/LICENSE).

## Repository

[https://github.com/t-fbd/cdg_api](https://github.com/t-fbd/cdg_api)
