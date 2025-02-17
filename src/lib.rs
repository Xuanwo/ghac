/// generated gRPC rust client for GitHub Actions Cache service V2.
///
/// This is the v1 gRPC version, the first version of the GitHub Actions Cache service V2.
///
/// ```rust
/// use prost::Message;
/// use ghac::v1::CreateCacheEntryRequest;
///
/// fn test() -> Result<(), Box<dyn std::error::Error>> {
///     let request = CreateCacheEntryRequest {
///         metadata: None,
///         key: "hello, world!".to_string(),
///         version: "1".to_string(),
///     };
///     let bs = request.encode_to_vec();
///     Ok(())
/// }
/// ```
pub mod v1;
