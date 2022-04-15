extern crate websocket;
extern crate image_base64;
use websocket::ClientBuilder;
use std::process::Command;
use std::net::TcpStream;
use std::io::prelude::*;

fn main(){

  std::thread::spawn(move ||{
    let comando = std::process::Command::new("raspivid").args([
        "-t","0"
    ]).arg("-l").args([
        "-o",
        "tcp://127.0.0.1:7878",
    ]).spawn().unwrap();
});

  fn conectar() -> TcpStream{
    match TcpStream::connect("127.0.0.1:7878"){
      Ok(s)=>{return s}
      Err(_)=>{
        println!("RECONECTANDO . . .");
        conectar()}
    }
  }
  let mut stream = conectar();

  loop{
    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer).unwrap();
    println!("BUFFER: {:?}",buffer);
    println!("LEN BUFFER: {}",buffer.len())
  }
}
