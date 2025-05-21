use crate::SharedData;
use axum::{
    debug_handler,
    extract::State,
    http::StatusCode,
    response::Html
    
};

use handlebars::{Handlebars, RenderError, RenderErrorReason};
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
struct TemplateData<T: Serialize> {
    parent: String,
    data: T
}

fn render_page<T>(hb: &Handlebars, page_name: &str, data: &T) -> Result<String, RenderError>
where 
    T: serde::Serialize
{
    let template_data = TemplateData {
        parent: "layouts/application.html".to_string(),
        data
    };
    hb.render(page_name, &template_data)
}

#[debug_handler]
pub async fn home_handler(State(data): State<SharedData>) -> (StatusCode, Html<String>) {
    let data = data.read();

    let page_data = json!({
        "title": "Welcome",
        "other_stuff": "testing other stuff",
    });

    match render_page(&data.hb, "index.html", &page_data) {
        Ok(r) => (StatusCode::OK , Html(r)),
        Err(e) => match e.reason() {
            RenderErrorReason::TemplateNotFound(_) => (StatusCode::NOT_FOUND, Html(data.error_pages.not_found.clone())),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, Html(data.error_pages.server_error.clone()))
        }
    }


}
