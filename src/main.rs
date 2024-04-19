// Uncomment this block to pass the first stage
use std::{
    env, fs,
    io::{Read, Write},
    net::TcpListener,
    thread,
};

fn main() {
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
    println!("reqeust: {}", request);
    let lines: Vec<&str> = request.split("\r\n").collect();
    if lines.is_empty() {
        return "HTTP/1.1 400 BAD REQUEST\r\n\r\n".to_string();
    }

    let req_target: Vec<&str> = lines[0].split_whitespace().collect();
    let method = req_target[0];
    let path = req_target[1];
    let _req_host: Vec<&str> = lines[1].split_whitespace().collect();
    let req_user_agent: Vec<&str> = lines[2].split_whitespace().collect();

    if method == "POST" && path.starts_with("/files/") {
        let filename = &path[7..];

        let args = env::args().collect::<Vec<String>>();
        let dir = args[2].clone();
        let filename = format!("{}/{}", dir, filename);
        println!("Filename: {}", filename);

        let request_body_start = request.find("\r\n\r\n").map(|i| i + 4).unwrap_or(request.len());
        let request_body = &request.as_bytes()[request_body_start..];
        let request_body_without_nulls = String::from_utf8_lossy(request_body)
            .replace("\x00", "")
            .into_bytes();

        match fs::write(&filename, request_body_without_nulls) {
            Ok(_) => return "HTTP/1.1 201 Created\r\n\r\n".to_string(),
            Err(e) => {
                eprintln!("Error writing file: {}", e);
                return "HTTP/1.1 500 Internal Server Error\r\n\r\n".to_string();
            }
        }
    }

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
