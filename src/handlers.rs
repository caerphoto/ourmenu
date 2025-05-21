use crate::SharedData;
use axum::{
    debug_handler,
    extract::{Path, State},
    http::{
        StatusCode,
        HeaderMap,
        HeaderValue,
        header
    },
    response::Html,
};
use handlebars::{Handlebars, RenderError, RenderErrorReason};
use mime::{
    TEXT_CSS,
    APPLICATION_JAVASCRIPT,
    APPLICATION_OCTET_STREAM
};
use serde::Serialize;
use serde_json::json;
use std::path::{
    Path as FsPath,
    PathBuf,
};

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

fn get_local_path<P: AsRef<FsPath>>(requested_path: P, exe_path: &PathBuf) -> PathBuf {

    log::info!("Requested path: {}", requested_path.as_ref().display());

    let mut local_path = PathBuf::from(exe_path);
    local_path.push("static/assets");
    local_path.push(requested_path);
    log::info!("File system path: {}", local_path.display());
    local_path
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

#[debug_handler]
pub async fn asset_handler(Path(path): Path<String>, State(data): State<SharedData>) -> (StatusCode, HeaderMap, Vec<u8>) {
    let local_path: PathBuf = {
        let data = data.read();
        get_local_path(&path, &data.content_dir)
    };

    let mut hm = HeaderMap::new();

    let bytes = if let Ok(b) = std::fs::read(&local_path) {
        b
    } else {
        hm.insert(header::CONTENT_TYPE, HeaderValue::from_static("text/html"));
        let error_page = data.read().error_pages.server_error.clone().into();
        return (StatusCode::INTERNAL_SERVER_ERROR, hm, error_page)
    };

    let mime = if let Some(ext) = local_path.extension() {
        match &*ext.to_string_lossy().to_string() {
            "css" => TEXT_CSS,
            "js" => APPLICATION_JAVASCRIPT,
            _ => APPLICATION_OCTET_STREAM
        }
    } else {
        APPLICATION_OCTET_STREAM
    };

    hm.insert(header::CONTENT_TYPE, HeaderValue::from_str(mime.as_ref()).expect("Failed to parse mime value"));
    (StatusCode::OK, hm, bytes)
}
