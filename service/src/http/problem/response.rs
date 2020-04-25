use super::problem::*;
use actix_web::{
  http::{header, StatusCode},
  Error, HttpRequest, HttpResponse, Responder, ResponseError,
};
use futures::future::{ready, Ready};
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

/// API Model representing an RFC-7807 Problem
#[derive(Serialize)]
struct ProblemModel {
  r#type: &'static str,
  title: String,
  status: u16,
  #[serde(skip_serializing_if = "Option::is_none")]
  detail: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  instance: Option<String>,
  #[serde(flatten)]
  extra: HashMap<String, Value>,
}

impl<T> From<&Problem<T>> for HttpResponse
where
  T: ProblemType,
{
  fn from(problem: &Problem<T>) -> HttpResponse {
    let problem_details = ProblemModel {
      r#type: problem.error.error_code(),
      title: format!("{}", problem.error),
      status: problem.status.as_u16(),
      detail: problem.detail.clone(),
      instance: problem.instance.clone(),
      extra: problem.extra.clone(),
    };

    HttpResponse::build(problem.status)
      .header(header::CONTENT_TYPE, "application/problem+json")
      .json(problem_details)
  }
}

impl<T> From<Problem<T>> for HttpResponse
where
  T: ProblemType,
{
  fn from(problem: Problem<T>) -> HttpResponse {
    HttpResponse::from(&problem)
  }
}

impl<T> Responder for Problem<T>
where
  T: ProblemType,
{
  type Error = Error;
  type Future = Ready<Result<HttpResponse, Error>>;

  fn respond_to(self, _req: &HttpRequest) -> Self::Future {
    // Create response and set content type
    ready(Ok(self.into()))
  }
}

impl<T> ResponseError for Problem<T>
where
  T: ProblemType,
{
  fn status_code(&self) -> StatusCode {
    self.status
  }

  fn error_response(&self) -> HttpResponse {
    self.into()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[derive(thiserror::Error, Debug, PartialEq)]
  pub enum ProblemDetails {
    #[error("Something Happened")]
    SomeProblem,
  }

  impl ProblemType for ProblemDetails {
    fn error_code(&self) -> &'static str {
      "tag:spacegame,2020:some/problem"
    }
  }

  #[test]
  fn test_basic_problem_to_response() {
    let problem = Problem::new(ProblemDetails::SomeProblem, StatusCode::BAD_REQUEST);

    let response: HttpResponse = problem.into();
    assert_eq!(StatusCode::BAD_REQUEST, response.status());
    assert_eq!(
      "application/problem+json",
      response.headers().get(header::CONTENT_TYPE).unwrap()
    );
    // TODO: Assert response body
  }
}
