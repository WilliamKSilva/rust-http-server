use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

mod thread_pool;

fn main() {
    let listener = match TcpListener::bind("127.0.0.1:3000") {
        Ok(tcp_listener) => {
            println!("Listening on port 3000");
            tcp_listener
        }
        Err(error) => panic!("{:?}", error),
    };

    let pool = thread_pool::ThreadPool::new(4);

    for tcp_stream_result in listener.incoming() {
        let stream = tcp_stream_result.unwrap(); 

        pool.execute(|| {
            handle_stream(stream);
        });
    }
}

fn handle_stream(mut stream: TcpStream) {
    let mut buff = [0; 1000];

    match stream.read(&mut buff) {
        Ok(bytes) => bytes,
        Err(error) => panic!("{:?}", error),
    };

    let data = match String::from_utf8(buff.to_vec()) {
        Ok(body) => body,
        Err(error) => panic!("{:?}", error),
    };

    match stream.shutdown(std::net::Shutdown::Both) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };

    println!("{:?}", data);
}
