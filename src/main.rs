mod thread_pool;

use std::{net::{TcpListener, TcpStream}, io::Read};

fn main() {
  let mut pool = thread_pool::ThreadPool::new();

  let listener = match TcpListener::bind("127.0.0.1:3000") {
    Ok(tcp_listener) => {
      println!("Listening on port 3000");
        tcp_listener
      },
    Err(error) => panic!("{:?}", error)
  };
}

fn accept_client_connection(listener: TcpListener, pool: &mut thread_pool::ThreadPool) {
  for tcp_stream_result in listener.incoming() {
    // match pool.first_avaiable() {
    //   Some(builder) => builder.spawn(|| {

    //   }),
    //   None => panic!("teste")
    // };

    // let mut handler = match pool.first_avaiable() {
    //   Some(thread) => thread.spawn(|| {
    //     println!("Teste");
    //   }),
    //   None => panic!("fudeu")
    // };

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