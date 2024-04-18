// Uncomment this block to pass the first stage
use std::{
    env, fs,
    io::{Read, Write},
    net::TcpListener,
    thread,
};

// const CRLF: &str = "";
// const RESPONSE_200: &str = "HTTP/1.1 200 OK\r\n\r\n";
// const RESPONSE_404: &str = "HTTP/1.1 404 Not Found\r\n\r\n";


fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                thread::spawn(move || {
                    let mut buffer = [0; 1024];
                    stream.read(&mut buffer).unwrap();
                    let request = String::from_utf8_lossy(&buffer[..]);
                    let response = build_response(request.to_string());
                    stream.write(response.as_bytes()).unwrap();
                    stream.flush().unwrap();
                    1
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn build_response(request: String) -> String {
    let lines: Vec<&str> = request.split("\r\n").collect();
    if lines.is_empty() {
        return "HTTP/1.1 400 BAD REQUEST\r\n\r\n".to_string();
    }

    let req_target: Vec<&str> = lines[0].split_whitespace().collect();
    let path = req_target[1];
    let _req_host: Vec<&str> = lines[1].split_whitespace().collect();
    let req_user_agent: Vec<&str> = lines[2].split_whitespace().collect();

    if path == "/" {
        return "HTTP/1.1 200 OK\r\n\r\n".to_string();
    }
    if path == "/user-agent" {
        let user_agent = req_user_agent[1];
        return format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            user_agent.len(),
            user_agent
        );
    } else if path.starts_with("/echo/") {
        let random_string = path.trim_start_matches("/echo/");
        return format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            random_string.len(),
            random_string
        );
    } else if path.starts_with("/files") {
        let filename = &path[7..];
        let args = env::args().collect::<Vec<String>>();
        let dir = args[2].clone();
        let filename = format!("{}/{}", dir, filename);
        println!("Reading file: {}", filename);
        let file = fs::read_to_string(filename);
        match file {
            Ok(fc) => {
                println!("File opened successfully");
                return format!("HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}\r\n", fc.len(), fc.to_string());
            }
            Err(error) => {
                println!("Error opening file: {}", error);
                return "HTTP/1.1 404 Not Found\r\n\r\n".to_string();
            }
        }
    }

    return "HTTP/1.1 404 NOT FOUND\r\n\r\n".to_string();
}
