use axum::{
    routing::{get, post},
    Router,
};

use crate::state::AppState;

mod create;
mod current;
mod delete;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(current::handler)
                .post(create::handler)
                .delete(delete::handler),
        )
        .route("/delete", post(delete::handler))
}
