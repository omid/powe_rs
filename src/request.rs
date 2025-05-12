use std::{
    io::{BufRead as _, BufReader, Read as _},
    net::TcpStream,
};

#[derive(Debug)]
pub struct Request {
    pub request: String,
    pub body: Option<String>,
}

impl Request {
    pub fn from_stream(stream: &mut TcpStream) -> std::io::Result<Self> {
        let mut reader = BufReader::new(stream);

        // Read request line
        let mut request = String::new();
        reader.read_line(&mut request)?;

        // Read headers and find content length
        let mut content_length = 0;
        loop {
            let mut line = String::new();
            reader.read_line(&mut line)?;
            if line == "\r\n" {
                break;
            }
            if line.to_ascii_lowercase().starts_with("content-length:") {
                content_length = line.split(':').nth(1).and_then(|s| s.trim().parse::<usize>().ok()).unwrap_or(0);
            }
        }

        // Read body
        let body = if content_length > 0 {
            let mut body = vec![0; content_length];
            reader.read_exact(&mut body)?;
            Some(String::from_utf8_lossy(&body).to_string())
        } else {
            None
        };

        Ok(Self { request, body })
    }
}
