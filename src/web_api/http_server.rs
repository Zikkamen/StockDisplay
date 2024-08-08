use std::{
    collections::{HashMap},
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use crate::web_api::api_register::ApiRegister;

pub struct HttpServer {
    listener: TcpListener,
    api_register: ApiRegister,
}

impl HttpServer {
    pub fn new(address: &str) -> Self {
        HttpServer{ 
            listener: TcpListener::bind(address).unwrap() ,
            api_register: ApiRegister::new(),
        }
    }

    pub fn start_listening(&self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(v) => self.handle_connection(v),
                Err(e) => println!("Error handling incoming request {}", e),
            };
        }
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let http_request: HashMap<String, String> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .map(|line| split_string_into_pairs(&line))
            .collect::<HashMap<String, String>>();
        
        println!("Handling Connection");
        self.api_register.handle_http_request(http_request, stream);
    }
}


fn split_string_into_pairs(s: &String) -> (String, String) {
    let n: usize = s.len();

    let sep_pos = match s.find(':') {
        Some(v) => v,
        None => return ("HEAD_REQUEST:".to_string(), s.clone()), 
    };

    let char_array = s.chars();
    
    (
        char_array.clone().take(sep_pos).collect(), 
        char_array.clone().skip(sep_pos+1).take(n-sep_pos).collect()
    )
}