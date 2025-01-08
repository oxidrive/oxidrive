use axum::{
    routing::{get, post},
    Router,
};

use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/accounts", post(accounts::create))
        .route("/session", get(session::get).post(session::create))
}

mod accounts;
mod session;
