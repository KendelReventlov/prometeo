extern crate websocket;
extern crate base64;
use websocket::ClientBuilder;
use std::process::Command;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

extern crate png;
use png::*;

fn handler(stream: TcpStream){
  let mut buffer = Vec::new();
  stream.read_to_end(&mut buffer).unwrap();
  println!("VEC: {:?}",buffer);
}

fn main(){

  let servidor = TcpListener::bind("127.0.0.1:3000").unwrap();
  let comando = std::process::Command::new("raspistill").args([
    "t",
    "0",
  ]).args([
    "-o",
    "tcp://127.0.0.1:3000",
  ]);
  for stream in servidor.incoming(){
    handler(stream.unwrap());
  }
  /*loop{

    let mut cliente = ClientBuilder::new("ws://192.168.100.10:3000/ws").unwrap().connect_insecure().unwrap();
    cliente.send_message(&websocket::Message::text(base64::encode(&buffer))).unwrap();
  }*/
}
