/// Settings needed to build the service
#[derive(Default)]
pub struct Settings {
  pub database_url: String,

  pub google_client_id: Option<String>,
  pub google_auth_url: Option<String>,
  pub google_redirect_url: Option<String>,
}
