use axum::{
    routing::get,
    Router,
};

use crate::SharedData;

use crate::handlers::{
    home_handler,
    asset_handler
};

pub fn init(shared_data: SharedData) -> Router {
    Router::new()
        .route("/", get(home_handler))
        .route("/assets/{*path}", get(asset_handler))
        .with_state(shared_data)
}
