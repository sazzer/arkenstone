use actix_web::test;
use insta::{assert_display_snapshot, assert_json_snapshot};

#[actix_rt::test]
#[cfg_attr(not(feature = "docker_tests"), ignore)]
async fn test_healthcheck() {
  let service = crate::service::Service::new().await;
  let res = service
    .request(test::TestRequest::get().uri("/health").to_request())
    .await;
  assert_display_snapshot!(res.headers());
  assert_json_snapshot!(res.to_json().unwrap());
}
