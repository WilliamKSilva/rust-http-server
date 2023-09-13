use std::{net::{TcpListener, TcpStream}, io::{Write, Read}};

fn main() {
    let listener = match TcpListener::bind("127.0.0.1:3000") {
        Ok(tcp_listener) => {
            println!("Listening on port 3000");

            tcp_listener
        },
        Err(error) => panic!("{:?}", error) 
    };

    for tcp_stream_result in listener.incoming() {
        let mut stream = match tcp_stream_result {
            Ok(buff) => buff,
            Err(error) => panic!("{:?}", error)
        };

        let body = handle_stream(&mut stream);

        println!("{:?}", body);
    }
}

fn handle_stream(stream: &mut TcpStream) -> String {
    println!("{:?}", stream.peer_addr());
    let mut buff = [0; 1000];

    match stream.read(&mut buff) {
        Ok(bytes) => bytes,
        Err(error) => panic!("{:?}", error)
    };

    let data = match String::from_utf8(buff.to_vec()) {
        Ok(body) => body,
        Err(error) => panic!("{:?}", error)
    };

    match stream.shutdown(std::net::Shutdown::Both) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };

    return data;
}