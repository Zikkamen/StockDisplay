mod value_store;
mod file_reader;
mod web_api;

use crate::web_api::http_server::HttpServer;

fn main() {
    let http_server = HttpServer::new("127.0.0.1:7878");
    http_server.start_listening();
}
