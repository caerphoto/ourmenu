mod routes;
mod handlers;


use handlebars::Handlebars;
use std::{net::IpAddr, path::PathBuf};
use tokio::net::TcpListener;
use std::{
    fs,
    sync::Arc
};
use parking_lot::RwLock;

#[derive(Clone,Debug)]
pub struct Config {
    listen_ip: String,
    listen_port: u16,
}

#[derive(Debug)]
pub struct ErrorPages {
    not_found: String,
    server_error: String
}

#[derive(Debug)]
pub struct CommonData {
    config: Config,
    hb: Handlebars<'static>,
    error_pages: ErrorPages,
    content_dir: PathBuf
}
pub type SharedData = Arc<RwLock<CommonData>>;

#[tokio::main]
async fn main() {
    env_logger::init();

    let mut hb = Handlebars::new();
    hb.register_templates_directory("templates", handlebars::DirectorySourceOptions::default())
        .expect("Failed to register Handlebars templates directory");

    let content_dir = std::env::current_dir().expect("Failed to get current directory");

    let common_data = CommonData {
        config: Config {
            listen_ip: String::from("127.0.0.1"),
            listen_port: 4000
        },
        hb,
        error_pages: ErrorPages {
            not_found: fs::read_to_string("static/not_found.html").expect("Failed to read not_found error page"),
            server_error: fs::read_to_string("static/server_error.html").expect("Failed to read server_error error page"),
        },
        content_dir
    };


    log::info!("Executable directory: {}", common_data.content_dir.display());
    log::debug!("Registered templates:");
    let templates = common_data.hb.get_templates();
    for key in templates.keys() {
        log::debug!("{}", key);
    }

    let initialization_config = common_data.config.clone();
    let shared_common_data = Arc::new(RwLock::new(common_data));

    let app = routes::init(shared_common_data.clone());

    let listen_ip = initialization_config
        .listen_ip
        .parse::<IpAddr>()
        .unwrap_or_else(|_| panic!("Failed to parse listen IP from {}", &initialization_config.listen_ip));
    let listen_port = initialization_config.listen_port;
    log::info!("Listening on http://{}:{}/ ...", &listen_ip, &listen_port);
    let listener = TcpListener::bind((listen_ip, listen_port)).await.expect("Failed to bind to IP");

    axum::serve(listener, app)
        .await
        .unwrap();

}
