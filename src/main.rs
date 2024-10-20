use std::fs;

use minijinja::{Environment, context};

use network_essentials::web_api::http_server::HttpServer;
use network_essentials::web_api::api_register::ApiRegister;
use network_essentials::web_api::api_register::HttpConnectionDetails;

static SERVER_IP:&'static str = "localhost";

fn main() {
    let mut main_api = ApiRegister::new(index);

    let files_api = ApiRegister::new(return_file);
    main_api.register_prefix("/files", files_api);

    let stocks_api = ApiRegister::new(index_stock);
    main_api.register_prefix("/stocks", stocks_api);

    let forecasts_api = ApiRegister::new(index_forecasts);
    main_api.register_prefix("/forecasts", forecasts_api);

    let http_server = HttpServer::new("127.0.0.1:7878", main_api, 16, 100);
    http_server.start_listening();
}

fn index(_http_request: HttpConnectionDetails) -> String {
    let html_file_content = fs::read_to_string("./files/static/html/bootstrap.html").expect("Valid file");
    let mut env = Environment::new();

    let _ = env.add_template("home", &html_file_content);
    let _ = env.add_template("server_ip", SERVER_IP);

    let mut stock_list:Vec<String> = Vec::new();
    
    for i in 0..50 { 
        stock_list.push(format!("Stock{}", i)); 
    }

    let tmpl = env.get_template("home").expect("Valid template");
    let contents = tmpl.render(context!(
        stock_list => stock_list,
        server_ip => SERVER_IP,
    )).expect("Valid rendering template");

    write_respone_successful(contents)
}

fn index_forecasts(_http_request: HttpConnectionDetails) -> String {
    let html_file_content = fs::read_to_string("./files/static/html/forecasts.html").expect("Valid file");
    let mut env = Environment::new();

    let _ = env.add_template("home", &html_file_content);
    let _ = env.add_template("server_ip", SERVER_IP);

    let mut stock_list:Vec<String> = vec!["Stock_1".to_owned(), "Stock2".to_owned()];
    
    for i in 0..50 { 
        stock_list.push(format!("Stock{}", i)); 
    }

    let tmpl = env.get_template("home").expect("Valid template");
    let contents = tmpl.render(context!(
        stock_list => stock_list,
        server_ip => SERVER_IP,
    )).expect("Valid rendering template");

    write_respone_successful(contents)
}

fn index_stock(http_request: HttpConnectionDetails) -> String {
    let html_file_content = fs::read_to_string("./files/static/html/stock_single_view.html").expect("Valid file");
    let mut env = Environment::new();

    let _ = env.add_template("stock", &html_file_content);

    let path = http_request.get_path();

    let stock_name:String = match path.split('/').next_back() {
        Some(v) => v.to_uppercase(),
        None => return write_respone_404(),
    };

    let tmpl = env.get_template("stock").expect("Valid template");
    let contents = tmpl.render(context!(
        stock_name => stock_name,
        server_ip => SERVER_IP,
    )).expect("Valid rendering template");

    write_respone_successful(contents)
}

fn return_file(http_request: HttpConnectionDetails) -> String {
    let path = http_request.get_path();
    let file_path = ".".to_owned() + &path.clone();

    let file = fs::read_to_string(file_path.clone());

    match file {
        Ok(v) => write_respone_successful(v),
        Err(_) => {
            println!("File not found {}", file_path);

            write_respone_404()
        },
    }
}

fn write_respone_successful(contents: String) -> String {
    let status_line = "HTTP/1.1 200 OK";
    let length = contents.len();
    
    format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}")
}

fn write_respone_404() -> String {
    let status_line = "HTTP/1.1 404";
    
    format!("{status_line}\r\n")
}