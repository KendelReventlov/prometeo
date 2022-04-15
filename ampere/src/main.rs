extern crate websocket;
extern crate image_base64;
use websocket::ClientBuilder;
use std::process::Command;
use std::net::TcpStream;
use std::io::prelude::*;

mod executer;

fn main(){

  executer::iniciar_tcp_camara();
  /*Command::new("raspivid").args(["-t","0"]).arg("-l").args([
    "-o",
    "tcp://127.0.0.1:7878",
  ]);
  std::thread::sleep_ms(10000);
  */let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
  let mut buffer = [0;128];
  loop{
    stream.read(&mut buffer).unwrap();
    println!("BUFFER: {:?}",buffer);
    println!();
  }
  /*loop{
    let mut cliente = ClientBuilder::new("ws://192.168.100.10:3000/ws").unwrap().connect_insecure().unwrap();
    cliente.send_message(&websocket::Message::text("hola")).unwrap();
  }*/
}
