use actix_web::http::{HeaderMap, StatusCode};
use bytes::Bytes;
use serde_json::Value;

/// The response from a test request to the server
pub struct TestResponse {
  pub status: StatusCode,
  pub headers: HeaderMap,
  pub body: Bytes,
}

impl TestResponse {
  /// Build the headers of the response
  pub fn headers(&self) -> String {
    let headers = self
      .headers
      .iter()
      .map(|(name, value)| format!("{}: {}", name, value.to_str().unwrap()))
      .collect::<Vec<String>>()
      .join("\n");

    format!("HTTP/1.1 {}\n{}", self.status, headers)
  }

  /// Convert the response body to JSON
  pub fn to_json(&self) -> Result<Value, serde_json::error::Error> {
    serde_json::from_slice(&self.body)
  }
}
