extern crate websocket;
extern crate base64;
use websocket::ClientBuilder;
use std::process::Command;
use std::net::TcpStream;
use std::io::prelude::*;

extern crate png;
use png::*;

fn main(){

  std::thread::spawn(move ||{
    let comando = std::process::Command::new("raspivid").args([
        "-t","0"
    ]).arg("-l").args([
      "-w","480",
    ]).args([
      "-h","320",
    ]).args([
        "-o",
        "tcp://127.0.0.1:7878",
    ]).spawn().unwrap();
});

  fn conectar() -> TcpStream{
    match TcpStream::connect("127.0.0.1:7878"){
      Ok(s)=>{return s}
      Err(_)=>{conectar()}
    }
  }
  let mut stream = conectar();


  
  loop{
    let mut buffer = [0;5000];
    stream.read(&mut buffer).unwrap();
    println!("BUFFER: {:?}",buffer);
    println!("LEN BUFFER: {}",buffer.len());

    let mut cliente = ClientBuilder::new("ws://192.168.100.10:3000/ws").unwrap().connect_insecure().unwrap();
    cliente.send_message(&websocket::Message::text(base64::encode(&buffer))).unwrap();
  }
}
