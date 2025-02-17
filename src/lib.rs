/// generated gRPC rust client for GitHub Actions Cache service V2.
///
/// This is the v1 gRPC version, the first version of the GitHub Actions Cache service V2.
///
/// ```rust
/// use ghac::v1::cache_service_client::CacheServiceClient;
/// use ghac::v1::CreateCacheEntryRequest;
///
/// async fn test() -> Result<(), Box<dyn std::error::Error>> {
///     let mut client = CacheServiceClient::connect("http://127.0.0.1:1234").await?;
///
///     let request = tonic::Request::new(CreateCacheEntryRequest {
///         metadata: None,
///         key: "hello, world!".to_string(),
///         version: "1".to_string(),
///     });
///
///     let response = client.create_cache_entry(request).await?;
///     Ok(())
/// }
/// ```
pub mod v1;
