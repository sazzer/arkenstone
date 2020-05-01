use actix_http::Request;

/// Wrapper around the service being tested
pub struct Service<'db> {
    _db: arkenstone_lib::testing::database::TestDatabase<'db>,
    service: arkenstone_lib::Service,
}

impl<'db> Service<'db> {
    pub async fn new() -> Service<'db> {
        let db = arkenstone_lib::testing::database::TestDatabase::default();

        let settings = arkenstone_lib::Settings {
            database_url: db.url.clone(),
            ..arkenstone_lib::Settings::default()
        };

        let service = arkenstone_lib::Service::new(&settings).await;

        Service { _db: db, service }
    }

    /// Send a request to the service and return the response
    pub async fn request(&self, req: Request) -> arkenstone_lib::TestResponse {
        log::debug!("Making request: {:?}", req);
        self.service.test_request(req).await
    }
}
