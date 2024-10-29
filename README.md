# cdg_api

A simple library to interact with the [US Congress API](https://api.congress.gov/).

![Crates.io](https://img.shields.io/crates/v/cdg_api)
![MIT License](https://img.shields.io/crates/l/cdg_api)

## Overview

This library provides a Rust interface for the US Congress API, allowing you to construct API endpoints, build URLs with parameters, and retrieve legislative data.

## Features

- Enumerated endpoints covering various legislative data.
- Parameter models for customizing API queries.
- URL builders for constructing API URLs with parameters.
- Utility function to fetch and pretty-print JSON responses (requires `curl` and `jq`).

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

- This example requires `curl` and `jq` to be installed on your system.
- Ensure that the `CDG_API_KEY` environment variable is set as described above.

```rust
use std::process::exit;

use cdg_api::{
    url_builders::get_endpoint_url,
    Endpoints, NewEndpoint,
    param_models::{FormatType, MemberListParams},
    curl_and_jq,
};

fn main() {
    let params = MemberListParams {
        format: Some(FormatType::Json),
        limit: Some(10),
        current_member: Some(true),
        ..MemberListParams::default()
    };

    let endpoint = Endpoints::new_member_list(params);

    println!("{}", get_endpoint_url(endpoint.clone()));

    match curl_and_jq(
        &get_endpoint_url(endpoint),
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

## Functions

### `curl_and_jq`

Utility function that performs a `curl` request and processes the JSON output with `jq`.

```rust
pub fn curl_and_jq(url: &str, jq_filter: &str) -> Result<(), Box<dyn std::error::Error>>;
```

- **Parameters**:
  - `url`: The API endpoint URL.
  - `jq_filter`: A `jq` filter string to process the JSON output.

**Note**:

- Requires `curl` and `jq` to be installed.
- Ensure that the `CDG_API_KEY` environment variable is set.

### Example

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

In the above code:

- The line `// Ensure CDG_API_KEY is set in your environment` reminds you to set the API key via environment variables.
- The actual setting of the API key is omitted to avoid hardcoding sensitive information.

## Environment Variables

- **CDG_API_KEY**: Your API key for the US Congress API.

## Future Work

We plan to enhance the library by implementing response models for parsing API responses into Rust data structures. This will allow users to work with strongly typed data rather than raw JSON, improving the usability and safety of the library.

- **Response Models**: Define `structs` that map to the API response JSON structures using `serde` for deserialization.
- **Async Support**: Introduce asynchronous HTTP request support for better performance in applications that can leverage it.
- **Error Handling**: Improve error handling mechanisms to provide more informative feedback to users.

Contributions are welcome! If you're interested in helping out, feel free to open an issue or submit a pull request on our [GitHub repository](https://github.com/t-fbd/cdg_api).

## License

This project is licensed under the terms of the [MIT license](LICENSE).

## Repository

[https://github.com/t-fbd/cdg_api](https://github.com/t-fbd/cdg_api)

