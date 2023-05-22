<p align="center">
<img src="https://app.emailvalidation.io/img/logo/emailvalidation.png" width="300"/>
</p>

# emailvalidation-rs: Rust geolocation service via emailvalidation.io

This package is a Rust wrapper for [emailvalidation.io](https://emailvalidation.io) that aims to make the usage of the API as easy as possible in your project. Emailvalidation is an email validation API that enables you to improve your marketing campaigns & conversion rates!


## Installation

This crate is under development. Especially the response parsing needs some more testing. However, if you still want to use it, you can install it by adding this to your `Cargo.toml`:

```toml
[dependencies]
emailvalidation = "0.1.1"
```

## Requirements

1. API Key for [emailvalidation.io](https://emailvalidation.io)
2. Async runtime like [tokio](https://crates.io/crates/tokio)

## Quickstart

```rust
use emailvalidation::Emailvalidation;
use emailvalidation::models;

async fn request_latest() -> Result<models::DetailsResponse, ipbase::Error> {
    let emailvalidation_api = Emailvalidation::new("<your-api-key>")?;
    let details = emaailvalidation_api.info("john@doe.com").await?;
     Ok(details)
}
```

Find out more about our endpoints, parameters and response data structure in the [docs]

## License

The MIT License (MIT). Please see [License File](LICENSE.md) for more information.

[docs]: https://emailvalidation.io/docs
[emailvalidation.io]: https://emailvalidation.io