use actix_web::{http::StatusCode, test};
use galvanic_assert::{
    assert_that,
    matchers::{variant::*, *},
};
use std::collections::HashMap;

#[actix_rt::test]
#[cfg_attr(not(feature = "docker_tests"), ignore)]
async fn test_start_google_provider() {
    let service = crate::service::Service::new().await;
    let res = service
        .request(
            test::TestRequest::get()
                .uri("/authentication/google")
                .to_request(),
        )
        .await;

    assert_that!(&res.status, eq(StatusCode::FOUND));

    let location = res.headers.get("location").unwrap();
    let parsed_location = url::Url::parse(location.to_str().unwrap()).unwrap();
    assert_that!(&parsed_location.scheme(), eq("https"));
    assert_that!(
        &parsed_location.host_str(),
        maybe_some(eq("accounts.google.com"))
    );
    assert_that!(&parsed_location.path(), eq("/o/oauth2/v2/auth"));
    let query_params: HashMap<String, String> = parsed_location
        .query_pairs()
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .collect();
    assert_that!(&query_params.len(), eq(5));
    assert_that!(
        &query_params
            .get(&"redirect_uri".to_owned())
            .map(Clone::clone),
        maybe_some(eq(
            "http://localhost:8000/authentication/google/complete".to_owned()
        ))
    );
    assert_that!(
        &query_params.get(&"scope".to_owned()).map(Clone::clone),
        maybe_some(eq("openid email profile".to_owned()))
    );
    assert_that!(
        &query_params
            .get(&"response_type".to_owned())
            .map(Clone::clone),
        maybe_some(eq("code".to_owned()))
    );
    assert_that!(
        &query_params.get(&"client_id".to_owned()).map(Clone::clone),
        maybe_some(eq("googleClientId".to_owned()))
    );
    assert_that!(
        &query_params.get(&"state".to_owned()).map(Clone::clone),
        maybe_some(any_value())
    );
}
