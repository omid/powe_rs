use std::io::{Read as _, Write as _};

mod systemctl;
use systemctl::SystemCtl;

const HTML: &str = include_str!("index.html");
const SERVICE_NAME: &str = "powe_rs.service";
const BINARY_PATH: &str = "/usr/local/bin/powe_rs";
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn serve(ip: &str, port: u16) {
    let bind_addr = if ip.contains(":") {
        // IPv6
        format!("[{}]:{}", ip, port)
    } else {
        // IPv4
        format!("{}:{}", ip, port)
    };
    let listener = std::net::TcpListener::bind(&bind_addr).expect("Failed to bind address");
    println!("Listening on http://{}:{}", ip, port);
    for mut stream in listener.incoming().flatten() {
        std::thread::spawn(move || {
            let mut buffer = [0; 1024];
            let _ = stream.read(&mut buffer);
            let request = String::from_utf8_lossy(&buffer);

            let response = if request.contains("POST /poweroff") {
                let when = extract_body(&request);
                SystemCtl::poweroff(when.as_deref());
                "HTTP/1.1 204 NO CONTENT".to_string()
            } else if request.contains("POST /reboot") {
                let when = extract_body(&request);
                SystemCtl::reboot(when.as_deref());
                "HTTP/1.1 204 NO CONTENT".to_string()
            } else {
                format!("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n{}", HTML)
            };
            let _ = stream.write(response.as_bytes());
        });
    }
}

fn extract_body(request: &str) -> Option<String> {
    if let Some(idx) = request.find("\r\n\r\n") {
        let body = &request[idx + 4..];
        let val = body.trim_matches(|c: char| c == '\0' || c.is_whitespace());
        if !val.is_empty() {
            return Some(val.to_string());
        }
    }
    None
}

fn require_root_or_exit(cmd: &str) {
    let user = std::env::var("USER").unwrap_or_default();
    if user != "root" {
        eprintln!("Error: {} must be run as root.", cmd);
        std::process::exit(1);
    }
}

fn install_service(ip: &str, port: u16) {
    require_root_or_exit("install");
    let sysctl = SystemCtl::install_service(SERVICE_NAME, BINARY_PATH, ip, port);
    sysctl.enable();
    sysctl.stop();
    std::fs::copy(&std::env::current_exe().unwrap(), BINARY_PATH).expect("Failed to copy binary to /usr/local/bin");
    sysctl.start();
    println!("Service installed and started on http://{}:{}", ip, port);
}

fn uninstall_service(remove_binary: bool) {
    require_root_or_exit("uninstall");
    SystemCtl::uninstall_service(SERVICE_NAME);
    if remove_binary {
        std::fs::remove_file(BINARY_PATH).ok();
        println!("Service and binary removed.");
    } else {
        println!("Service removed.");
    }
}

fn parse_listen(args: &[String]) -> (String, u16) {
    let listen = args
        .iter()
        .position(|x| x == "-l" || x == "--listen")
        .and_then(|i| args.get(i + 1))
        .cloned()
        .unwrap_or_else(|| "0.0.0.0:54321".to_string());
    let (ip, port) = if listen.starts_with('[') {
        // IPv6: [ip]:port
        if let Some(end) = listen.find(']') {
            let ip = listen[1..end].to_string();
            let port = listen[end + 2..].parse().unwrap_or(54321);
            (ip, port)
        } else {
            ("::".to_string(), 54321)
        }
    } else if listen.matches(':').count() > 1 {
        // IPv6 without brackets, assume full address then :port
        if let Some(idx) = listen.rfind(':') {
            let ip = listen[..idx].to_string();
            let port = listen[idx + 1..].parse().unwrap_or(54321);
            (ip, port)
        } else {
            (listen, 54321)
        }
    } else if listen.contains(':') {
        // IPv4: ip:port
        let parts: Vec<&str> = listen.split(':').collect();
        let ip = parts[0].to_string();
        let port = parts[1].parse().unwrap_or(54321);
        (ip, port)
    } else {
        ("0.0.0.0".to_string(), listen.parse().unwrap_or(54321))
    };
    (ip, port)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 || args.iter().any(|x| x == "--help" || x == "-h") {
        print_help();
        return;
    }
    match args[1].as_str() {
        "serve" => {
            let (ip, port) = parse_listen(&args);
            serve(&ip, port);
        }
        "install" => {
            let (ip, port) = parse_listen(&args);
            install_service(&ip, port);
        }
        "uninstall" => {
            let remove_binary = args.iter().any(|x| x == "-a");
            uninstall_service(remove_binary);
        }
        _ => {
            print_help();
        }
    }
}

fn print_help() {
    println!(
        r#"powe_rs - Simple Power Control Web UI
Version: {VERSION}
powe_rs is a simple web server that allows you to power off or reboot your system via browser.

USAGE:
  powe_rs serve [-l <IP:PORT>|<PORT>]
  powe_rs install [-l <IP:PORT>|<PORT>]
  powe_rs uninstall [-a]
  powe_rs -h

COMMANDS:
  serve                  Start the web server
  install                Install as a system-wide systemd service and enable it
  uninstall              Remove the systemd service (and binary if -a is given).

OPTIONS:
  -l <IP:PORT>|<PORT>    Set the IP address and port to listen on for example: 127.0.0.1:54321 or [::]:54321 for IPv6 (default: 0.0.0.0:54321)
  -a                     Also remove the powe_rs binary
  -h                     Show this help message"#
    );
}
