# cdg_api

![Crates.io](https://img.shields.io/crates/v/cdg_api)
![MIT License](https://img.shields.io/crates/l/cdg_api)

A simple Rust library to interact with the [US Congress API](https://api.congress.gov/).

## Installation

Add `cdg_api` to your `Cargo.toml`:

```toml
[dependencies]
cdg_api = "1.3.6"
```

Or use `cargo` to add the dependency:

```bash
cargo add cdg_api
```

If you don't want to pull in reqwest as a dependency, you can disable the `requests` feature by just disabling the default features:

```toml
[dependencies]
cdg_api = { version = "1.3.6", default-features = false }
```

or

```bash
cargo add cdg_api --no-default-features
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

## Using `CongressApiClient`

[`CongressApiClient`] allows you to interact with various API endpoints. Below are examples demonstrating how to fetch different types of data, including the fetch-all `Endpoints::Generic` variant.

For more detailed information, see the [documentation](https://docs.rs/cdg_api/1.3.6/cdg_api/), the
[examples directory](examples/), and the [US Congress API documentation](https://github.com/LibraryOfCongress/api.congress.gov/).

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

    // Create the endpoint and define parameters
    let endpoint = Endpoints::new_member_list(
        MemberListParams::default()
            .format(FormatType::Json)
            .limit(10)
            .current_member(true)
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

#### Example 2: Using `GenericResponse` with `parse_response`

Fetch detailed information about a specific bill using `GenericResponse` and parse it into a specific response model.

```rust
use cdg_api::CongressApiClient;
use cdg_api::endpoints::{Endpoints, NewEndpoint};
use cdg_api::param_models::BillDetailsParams;
use cdg_api::cdg_types::{BillType, FormatType};
use cdg_api::response_models::{BillDetailsResponse, GenericResponse, serialize_response, parse_response};

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let client = CongressApiClient::new(None)?; // Use API key from environment

    // Define parameters for bill details
    let params = BillDetailsParams::default()
        .format(FormatType::Json);

    // Specify the bill to fetch (e.g., H.R. 1234 from the 118th Congress)
    let endpoint = Endpoints::new_bill_details(118, BillType::Hr, 148, params);

    // Fetch the data as GenericResponse
    let response: GenericResponse = client.fetch(endpoint)?;

    // Parse the response as BillDetailsResponse
    let bill_details: BillDetailsResponse = match parse_response(&response) {
        Ok(bill_details) => bill_details,
        Err(e) => {
            // If parsing fails to a specific primary response fails, the GenericResponse can be serialized
            // to see the response structure. This can help in debugging and creating a specific response model.
            // The argument `true` pretty prints the response, `false` prints the raw JSON.
            println!("Failed to parse response: {}", e);
            println!("Response:\n\n{}", serialize_response(&response, true)?);
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
The `Endpoint` created can then call `parse_response` to parse the response into a specific response model. If parsing fails, the `GenericResponse` can be serialized to see the response structure.

This is essentially the Endpoints equivalent of the GenericResponse example above. One for requests and the other for responses.

```rust
use cdg_api::CongressApiClient;
use cdg_api::endpoints::{Endpoints, NewEndpoint};
use cdg_api::response_models::{DailyCongressionalRecordResponse, GenericResponse, serialize_response, parse_response};
use cdg_api::param_models::GenericParams;
use cdg_api::cdg_types::*;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let client = CongressApiClient::new(None)?; // Use API key from environment

    // Manually specify the endpoint string and parameters
    let endpoint = Endpoints::new_generic(
      "daily-congressional-record".to_string(),
      GenericParams::default().format(FormatType::Json)
    );

    // Fetch the data as GenericResponse. Theoretically, you could use any endpoint string
    // and parameters here, but the response structure may not be known, we parse it into the
    // suspected response model after fetching. We do this by calling `parse_response`, it 
    // will return a specific response model if the structure is known, else it will return
    // the GenericResponse. This ensures that the response is always parsed, rather than throwing
    // an error if the structure is unknown or unexpected during the fetch.
    let response: GenericResponse = client.fetch(endpoint)?;

    // Parse the response into DailyCongressionalRecordResponse
    match parse_response::<DailyCongressionalRecordResponse, GenericResponse>(&response) {
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
            println!("Response:\n\n{}", serialize_response(&response, true)?);
        }
    }

    Ok(())
}
```

## Related Projects

- **loc_api** : A Rust library for interacting with the Library of Congress API. Less polished than `cdg_api`, barebones interface, but functional.
    - [crate](https://crates.io/crates/loc_api)
    - [repository](https://github.com/t-fbd/loc_api)

## License

This project is licensed under the terms of the [MIT license](https://github.com/t-fbd/cdg_api/blob/master/LICENSE).

## Repository

[https://github.com/t-fbd/cdg_api](https://github.com/t-fbd/cdg_api)

## Acknowledgements

The data is sourced from the [U.S. Congress API](https://api.congress.gov/).
See their repository for more information: [LibraryOfCongress/api.congress.gov](https://github.com/LibraryOfCongress/api.congress.gov/).

## Contact

For questions or feedback, please contact me on [github](https://www.github.com/t-fbd).

If you find this project helpful, consider donating [PayPal](https://paypal.me/imturn?country.x=US&locale.x=en_US).

