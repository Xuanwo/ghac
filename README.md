# ghac &emsp; [![Build Status]][actions] [![Latest Version]][crates.io]

[Build Status]: https://img.shields.io/github/actions/workflow/status/Xuanwo/ghac/ci.yml
[actions]: https://github.com/Xuanwo/ghac/actions?query=branch%3Amain
[Latest Version]: https://img.shields.io/crates/v/ghac.svg
[crates.io]: https://crates.io/crates/ghac

`ghac` is a generated gRPC rust client for GitHub Actions Cache service V2.

Please note:

- ghac is a private service, and its API may change at any time. If you encounter any issues, please report them.
- This crate only provides a generated gRPC client. For a high-level API, please refer to [opendal](https://github.com/apache/opendal), which allows access to all storage using the same API.

## Quick Start

```rust
use ghac::v1::cache_service_client::CacheServiceClient;
use ghac::v1::CreateCacheEntryRequest;
///
async fn test() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = CacheServiceClient::connect("http://127.0.0.1:1234").await?;
///
    let request = tonic::Request::new(CreateCacheEntryRequest {
        metadata: None,
        key: "hello, world!".to_string(),
        version: "1".to_string(),
    });
///
    let response = client.create_cache_entry(request).await?;
    Ok(())
}
```

## Contributing

Check out the [CONTRIBUTING.md](./CONTRIBUTING.md) guide for more details on getting started with contributing to this project.

## Getting help

Submit [issues](https://github.com/Xuanwo/ghac/issues/new/choose) for bug report or asking questions in [discussion](https://github.com/Xuanwo/ghac/discussions/new?category=q-a).

#### License

Licensed under <a href="./LICENSE">Apache License, Version 2.0</a>.
