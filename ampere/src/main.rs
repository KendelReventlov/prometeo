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

  let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
  loop{
    let mut buffer = [0;128];
    stream.read(&mut buffer).unwrap();
    println!("BUFFER: {:?}",buffer);
    std::fs::write(std::path::Path::new("uwu.png"), &buffer).unwrap();
    let mut cliente = ClientBuilder::new("ws://192.168.100.10:3000/ws").unwrap().connect_insecure().unwrap();
    let imagen = image_base64::to_base64("uwu.png");
    cliente.send_message(&websocket::Message::text(imagen)).unwrap();
  }
}
