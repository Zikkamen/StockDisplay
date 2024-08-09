use std::{
    error,
    thread,
    collections::{HashMap},
    net::{TcpStream},
};

type FnType = fn(HashMap<std::string::String, std::string::String>, TcpStream);

pub struct ApiRegister {
    default_func: FnType,
    path_map: HashMap<(String, String), FnType>,
    prefix_map: HashMap<String, Self>,
}

impl ApiRegister {
    pub fn new(default_func: FnType) -> Self {
        ApiRegister{ default_func: default_func, path_map:HashMap::new(), prefix_map:HashMap::new() }
    }

    pub fn handle_http_request(&self, http_request: HashMap<String, String>, stream: TcpStream) {
        match get_method_and_path(&http_request) {
            Ok((method, path)) => self.handle_request_with_meta_data(method, path, http_request, stream),
            Err(_e) => (),
        };
    }

    pub fn handle_request_with_meta_data(&self,
            method: String,
            path: String,
            http_request: HashMap<String, String>, 
            stream: TcpStream) {
        let extract_prefix:Vec<&str> = path.split('/').collect();

        if extract_prefix.len() > 1 {
            match self.prefix_map.get(extract_prefix[1]) {
                Some(v) => {
                    let mut remaining_path = String::new();

                    for i in 2..extract_prefix.len() {
                        remaining_path.push_str("/");
                        remaining_path.push_str(extract_prefix[i]);
                    }

                    if remaining_path.len() == 0 { remaining_path = "/".to_string(); }

                    v.handle_http_request(http_request, stream);

                    return;
                },
                None => (),
            }
        }

        let func: FnType = match self.path_map.get(&(method, path)) {
            Some(v) => *v,
            None => self.default_func,
        };

        thread::spawn(move || { func(http_request, stream) });
    }

    pub fn register_function(&mut self, method: &str, path: &str, function: FnType) {
        self.path_map.insert((method.to_string(), path.to_string()), function);
    }

    pub fn register_prefix(&mut self, prefix: &str, api_register: ApiRegister) {
        self.prefix_map.insert(prefix.to_string(), api_register);
    }
}

pub fn get_method_and_path(http_request: &HashMap<String, String>) -> Result<(String, String), Box<dyn error::Error + 'static>> {
    let header = http_request.get("HEAD_REQUEST:").expect("A Header is expected");
    let split_header:Vec<&str> = header.split(' ').collect();

    if split_header.len() < 2 { panic!("Header is malformed"); }

    Ok((split_header[0].to_string(), split_header[1].to_string()))
}