// Uncomment this block to pass the first stage
use std::{
    io::{BufRead, BufReader, Write},
    net::TcpListener,
    str::FromStr,
};

// const CRLF: &str = "";
const RESPONSE_200: &str = "HTTP/1.1 200 OK\r\n\r\n";
const RESPONSE_404: &str = "HTTP/1.1 404 Not Found\r\n\r\n";

#[derive(Debug, PartialEq)]
enum HttpMethod {
    GET,
    POST,
}
impl FromStr for HttpMethod {
    type Err = ();

    fn from_str(input: &str) -> Result<HttpMethod, Self::Err> {
        match input {
            "GET" => Ok(HttpMethod::GET),
            "POST" => Ok(HttpMethod::POST),
            _ => Err(()),
        }
    }
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection");
                handle_connection(_stream)
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: std::net::TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let user_agent_header: String = http_request
        .iter()
        .filter(|s| s.starts_with("User-Agent:"))
        .map(|s| s.split_whitespace().nth(1).unwrap())
        .collect::<Vec<_>>()
        .first()
        .unwrap()
        .to_string();

    let mut parts = http_request[0].split_whitespace();
    let _method: HttpMethod = HttpMethod::from_str(parts.next().unwrap()).unwrap();
    let req_endpoint = parts.next().unwrap();

    let response = handle_req(req_endpoint, user_agent_header);
    stream.write_all(response.as_bytes()).unwrap();
}

fn handle_req(req: &str, user_agent_header: String) -> String {
    if req.len() == 1 {
        RESPONSE_200.to_string()
    } else if req.starts_with("/echo") {
        make_resp_from_string(req.trim_start_matches("/echo/"))
    } else if req.starts_with("/user-agent") {
        make_resp_from_string(user_agent_header.as_str())
    } else {
        return RESPONSE_404.to_string();
    }
}

fn make_resp_from_string(resp: &str) -> String {
    let base_text = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: ".to_string();
    let content_length = resp.len();
    format!(
        "{} {}\r\n\r\n{}",
        base_text, content_length, resp
    )
}
