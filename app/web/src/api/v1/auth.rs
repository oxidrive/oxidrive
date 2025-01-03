use axum::{routing::get, Router};

use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/session", get(session::get).post(session::create))
}

mod session;
