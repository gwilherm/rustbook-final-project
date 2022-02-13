// STD
use std::io::prelude::*;
use std::fs;
use std::thread;
use std::time::Duration;
use std::error::Error;

// Crates
use cancellable_io::*;
#[cfg(unix)]
use signal_hook::{consts::SIGINT, consts::SIGTERM, iterator::Signals};
#[cfg(unix)]
use log::info;

// Internal
use hello::threadpool::ThreadPool;
use hello::config::Config;

#[cfg(unix)]
fn signal_handler_thread(canceller: Canceller) -> Result<(), Box<dyn Error>> {
    let mut signals = Signals::new(&[SIGINT, SIGTERM])?;
    thread::spawn(move || {
        for sig in signals.forever() {
            info!("Received signal {:?}", sig);
            canceller.cancel().unwrap();
        }
    });
    
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let config = Config::load("config.toml");
    
    let (listener, _canceller) = TcpListener::bind(config.server.to_string()).unwrap();
    let pool = ThreadPool::new(config.threadpool.workers); 

    #[cfg(unix)]
    signal_handler_thread(_canceller)?;
    
    for stream in listener.incoming() {
        let (stream,..) = stream?;
        pool.execute(|| {
            handle_connection(stream);
        });
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
