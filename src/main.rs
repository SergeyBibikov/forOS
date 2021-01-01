use std::net::{TcpListener};
use std::io::{Read,Write};

fn main() {
    println!("Spawning a thread");
    std::thread::spawn(move || {
        start()
    });
    loop{}
}

fn start(){
    let a = TcpListener::bind("localhost:8080").unwrap();
    let response = "HTTP/1.1 200 OK\n\r\n\rHi there!";
    for i in a.incoming(){
        let mut buf = [0;1024];
        let mut stream = i.unwrap();
        stream.read(&mut buf).unwrap();
        stream.write(response.as_bytes()).unwrap();
        println!("A connection received {:?}",stream);
    }
}
