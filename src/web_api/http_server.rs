use std::{
    collections::{HashMap},
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    error,
};

use crate::web_api::api_register::ApiRegister;

pub struct HttpServer {
    listener: TcpListener,
    api_register: ApiRegister,
}

impl HttpServer {
    pub fn new(address: &str, main_api_register: ApiRegister) -> Self {
        HttpServer{ 
            listener: TcpListener::bind(address).unwrap() ,
            api_register: main_api_register,
        }
    }

    pub fn start_listening(&self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(mut v) => match self.handle_connection(&mut v) {
                    Ok(hm) => {
                        println!("Handling Connection");
                        self.api_register.handle_http_request(hm, v);
                    },
                    Err(e) => println!("Error handling incoming request {}", e),
                },
                Err(e) => println!("Error handling incoming request {}", e),
            };
        }
    }

    fn handle_connection(&self, mut stream: &mut TcpStream) -> Result<HashMap<String, String>,  Box<dyn error::Error + 'static>> {
        let buf_reader = BufReader::new(&mut stream);
        let http_request: HashMap<String, String> = buf_reader
            .lines()
            .map(|result| match result { Ok(v) => v, Err(_e) => String::new()})
            .take_while(|line| !line.is_empty())
            .map(|line| split_string_into_pairs(&line))
            .collect::<HashMap<String, String>>();
        
        Ok(http_request)
    }
}

fn split_string_into_pairs(s: &String) -> (String, String) {
    let n: usize = s.len();

    if n == 0 { return (String::new(), String::new()); }

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