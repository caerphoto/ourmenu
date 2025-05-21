use axum::{
    routing::{get, post},
    Router,
};

use crate::SharedData;

use crate::handlers::{
    home_handler,
    session::{
        new_session_handler,
        create_session_handler
    },
    user::{
        edit_user_handler,
        update_user_handler
    },
    asset_handler
};

pub fn init(shared_data: SharedData) -> Router {
    Router::new()
        .route("/",                  get(home_handler))
        .route("/sessions/new",      get(new_session_handler))
        .route("/sessions/create",   post(create_session_handler))
        .route("/users/{id}/edit",   get(edit_user_handler))
        .route("/users/{id}/update", post(update_user_handler))
        .route("/assets/{*path}",    get(asset_handler))
        .with_state(shared_data)
}
