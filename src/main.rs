use std::{
    fs,
    collections::{HashMap},
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use minijinja::{Environment, context};

mod file_reader;
mod database_clients;
mod value_store;

use crate::value_store::credentials_store::CredentialsStore;
use crate::database_clients::postgres_client::PostgresClient;

fn main() {
    let credential_store = CredentialsStore::new("credentials/database_credentials.xml".to_string());
    let mut postgres_client:PostgresClient = PostgresClient::new(credential_store);

    postgres_client.get_all_tables();

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);

        break;
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: HashMap<String, String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .map(|line| split_string_into_pairs(&line))
        .collect::<HashMap<String, String>>();

    let status_line = "HTTP/1.1 200 OK";

    let stock_list = ["AAPL", "NVDA", "MSFT", "COIN", "TSLA", "GOOG", "NFLX"];

    let html_file_content = match fs::read_to_string("bootstrap.html") {
        Ok(v) => v,
        Err(e) => panic!("Error reading HTML file {}", e),
    };

    let mut env = Environment::new();
    env.add_template("home", &html_file_content);

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

    for (key, value) in http_request {
        println!("{key} {value}");
    }
}

fn split_string_into_pairs(s: &String) -> (String, String) {
    let n: usize = s.len();

    let sep_pos = match s.find(':') {
        Some(v) => v,
        None => return ("HEAD_REQUEST:".to_string(), s.clone()), 
    };

    let char_array = s.chars();
    
    (char_array.clone().take(sep_pos).collect(), char_array.clone().skip(sep_pos+1).take(n-sep_pos).collect())
}
