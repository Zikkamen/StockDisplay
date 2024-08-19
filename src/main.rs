use std::{
    fs,
    fs::File,
    collections::{HashMap},
    io::{prelude::*},
    net::{TcpStream},
};

use minijinja::{Environment, context};

mod web_api;

use crate::web_api::http_server::HttpServer;
use crate::web_api::api_register::ApiRegister;
use crate::web_api::api_register::get_method_and_path;

fn main() {
    let mut main_api = ApiRegister::new(index);

    let files_api = ApiRegister::new(return_file);
    main_api.register_prefix("files", files_api);

    let stocks_api = ApiRegister::new(index_stock);
    main_api.register_prefix("stocks", stocks_api);

    let http_server = HttpServer::new("127.0.0.1:7878", main_api);
    http_server.start_listening();
}

fn index(_http_request: HashMap<String, String>, stream: TcpStream) {
    let html_file_content = fs::read_to_string("bootstrap.html").expect("Valid file");
    let mut env = Environment::new();

    env.add_template("home", &html_file_content);

    let mut stock_list:Vec<String> = Vec::new();
    
    for i in 0..50 { stock_list.push(format!("Stock{}", i)); }

    let tmpl = env.get_template("home").expect("Valid template");
    let contents = tmpl.render(context!(stock_list => stock_list)).expect("Valid rendering template");

    write_respone_successful(stream, contents);
}

fn index_stock(http_request: HashMap<String, String>, stream: TcpStream) {
    let html_file_content = fs::read_to_string("stock_single_view.html").expect("Valid file");
    let mut env = Environment::new();

    env.add_template("stock", &html_file_content);

    let (method, path) = get_method_and_path(&http_request).expect("Method and Path got checked already");
    let stock_name:String = match path.split('/').next_back() {
        Some(v) => v.to_uppercase(),
        None => return,
    };

    let tmpl = env.get_template("stock").expect("Valid template");
    let contents = tmpl.render(context!(stock_name => stock_name)).expect("Valid rendering template");

    write_respone_successful(stream, contents);
}

fn return_file(http_request: HashMap<String, String>, stream: TcpStream) {
    let (method, path) = get_method_and_path(&http_request).expect("Method and Path got checked already");

    let file_path = ".".to_owned() + &path.clone();
    let file = fs::read_to_string(file_path.clone());
    
    if !file.is_ok() {
        println!("File not found {}", file_path);
        return;
    }

    let file_contents = file.unwrap();

    write_respone_successful(stream, file_contents);
}

fn write_respone_successful(mut stream: TcpStream, contents: String) {
    let status_line = "HTTP/1.1 200 OK";
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}