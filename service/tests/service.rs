use actix_http::Request;

/// Wrapper around the service being tested
pub struct Service<'db> {
    _db: arkenstone_lib::testing::database::TestDatabase<'db>,
    service: arkenstone_lib::Service,
}

impl<'db> Service<'db> {
    pub async fn new() -> Service<'db> {
        Service::new_with_settings(&mut |_| {}).await
    }

    pub async fn new_with_settings(
        config: &mut dyn FnMut(&mut arkenstone_lib::Settings),
    ) -> Service<'db> {
        let _ = env_logger::try_init();
        let db = arkenstone_lib::testing::database::TestDatabase::default();

        let mut settings = arkenstone_lib::Settings {
            database_url: db.url.clone(),

            google_client_id: Some("googleClientId".to_owned()),
            google_client_secret: Some("googleClientSecret".to_owned()),
            google_auth_url: None,
            google_token_url: None,
            google_redirect_url: Some(
                "http://localhost:8000/authentication/google/complete".to_owned(),
            ),
        };
        config(&mut settings);

        let service = arkenstone_lib::Service::new(&settings).await;

        Service { _db: db, service }
    }

    /// Send a request to the service and return the response
    pub async fn request(&self, req: Request) -> arkenstone_lib::TestResponse {
        log::debug!("Making request: {:?}", req);
        self.service.test_request(req).await
    }
}
