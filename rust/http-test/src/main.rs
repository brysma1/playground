use std::{
    error::Error,
    io::{BufRead, BufReader, BufWriter, Write},
    net::{TcpListener, TcpStream},
    thread,
};

#[derive(Debug)]
struct Request {
    method: String,
    uri: String,
    version: String,
    stream: TcpStream,
}

impl Request {
    fn new(stream: TcpStream) -> Self {
        Self {
            method: String::new(),
            uri: String::new(),
            version: String::new(),
            stream: stream,
        }
    }

    fn handle_stream(&mut self) {
        let mut reader = BufReader::new(&self.stream);
        let mut request_start_line = String::new();
        reader
            .read_line(&mut request_start_line)
            .expect("Could not read line from stream.");

        let mut request_line = request_start_line.split_whitespace();

        self.method = String::from(
            request_line
                .next()
                .expect("Could not parse method from request"),
        );

        self.uri = String::from(
            request_line
                .next()
                .expect("Could not uri method from request"),
        );

        self.version = String::from(
            request_line
                .next()
                .expect("Could not parse version from request"),
        );

        dbg!(&self);

        match self.method.as_str() {
            "GET" => {
                println!("got a get aaaaaaaaaaaaaaaaaaaa");
                self.resolve_request();
            }
            "HEAD" | "POST" | "PUT" | "DELETE" | "CONNECT" | "OPTIONS" | "TRACE" | "PATCH" => {
                println!("didn't got a get aaaaaaaaaaaaaaaaaaaa");
                self.resolve_nonvalid_request();
            }
            _ => {
                panic!("Malformed method");
            }
        };
    }

    fn resolve_nonvalid_request(&self) {
        let mut writer = BufWriter::new(&self.stream);
        dbg!(
            writer
                .write(b"HTTP/1.1 418 I'm a teapot\r\nContent-Lenght: 12\r\n\r\nI'm a teapot")
                .expect("Could not write into buffer")
        );
        dbg!(writer.flush().expect("Could not flush writter into stream"));
    }

    fn resolve_request(&self) {
        match self.version.as_str() {
            "HTTP/1.1" => {}
            _ => panic!("HTTP version not supported"),
        };

        let mut writer = BufWriter::new(&self.stream);

        let mut content = String::new();

        match self.uri.as_str() {
            "/yo" => content.push_str("mama"),
            "/man" => content.push_str("znahgf va lb zbhgu"),
            "/favicon.ico" => panic!("who cares about an icon?"),
            _ => {
                self.resolve_nonvalid_request();
                return;
            }
        };

        dbg!(
            writer
                .write(b"HTTP/1.1 200 Ok\r\n")
                .expect("Could not write into buffer")
        );

        dbg!(
            writer
                .write_fmt(format_args!(
                    "Content-Type: text\r\nContent-Length: {}\r\n\r\n{}",
                    content.len(),
                    content
                ))
                .expect("Could not write into buffer")
        );

        dbg!(String::from_utf8_lossy(writer.buffer()));

        dbg!(writer.flush().expect("Could not flush writter into stream"));
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming() {
        thread::spawn(move || match stream {
            Ok(s) => {
                let mut request = Request::new(s);
                request.handle_stream();
            }
            Err(err) => {
                dbg!(thread::current());
                panic!("{err}")
            }
        });
    }

    Ok(())
}
