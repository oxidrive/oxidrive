use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(tag = "type")]
pub enum CsrfConfig {
    Cookie(CsrfCookieConfig),
    #[default]
    Fetch,
    None,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CsrfCookieConfig {
    #[serde(default = "default_cookie_name")]
    pub cookie_name: String,
}

#[inline]
fn default_cookie_name() -> String {
    "oxidrive_csrf_token".into()
}
