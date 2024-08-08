use std::{
    fs,
    thread,
    collections::{HashMap},
    io::{prelude::*},
    net::{TcpStream},
};

use minijinja::{Environment, context};

type FnType = fn(HashMap<std::string::String, std::string::String>, TcpStream);

pub struct ApiRegister {
    path_map: HashMap<(String, String), FnType>,
}

impl ApiRegister {
    pub fn new() -> Self {
        let mut api_register = ApiRegister{ path_map:HashMap::new() };
        
        api_register.register_functions();

        api_register
    }

    pub fn handle_http_request(&self, http_request: HashMap<String, String>, stream: TcpStream) {
        let header = http_request.get("HEAD_REQUEST:");

        if header.is_none() { return; }

        let split_header:Vec<&str> = header.unwrap().split(' ').collect();

        if split_header.len() < 2 { return; }

        let function = self.path_map.get(&(split_header[0].to_string(), split_header[1].to_string()));


        if function.is_some() {
            let func: FnType = *function.unwrap();

            thread::spawn(move || { func(http_request, stream) });
        }
    }

    fn register_functions(&mut self) {
        self.register("GET", "/", index);
    }

    fn register(&mut self, method: &str, path: &str, function: FnType) {
        self.path_map.insert((method.to_string(), path.to_string()), function);
    }
}

fn index(_http_request: HashMap<String, String>, mut stream: TcpStream) {
    let status_line = "HTTP/1.1 200 OK";
    
    let html_file_content = match fs::read_to_string("bootstrap.html") {
        Ok(v) => v,
        Err(e) => panic!("Error reading HTML file {}", e),
    };

    let mut env = Environment::new();
    match env.add_template("home", &html_file_content){
        Ok(_v) => (),
        Err(e) => panic!("Error adding template {}", e),
    };

    let mut stock_list:Vec<String> = Vec::new();
    
    for i in 0..50 {
        stock_list.push(format!("Stock{}", i));
    }

    let tmpl = match env.get_template("home") {
        Ok(v) => v,
        Err(e) => panic!("Error getting template {}", e),
    };

    let contents = match tmpl.render(context!(stock_list => stock_list)) {
        Ok(v) => v,
        Err(e) => panic!("Error rendering template {}", e),
    };

    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}