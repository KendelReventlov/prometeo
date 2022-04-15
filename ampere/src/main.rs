extern crate websocket;
extern crate base64;
use websocket::ClientBuilder;
use std::process::Command;
use std::net::TcpStream;
use std::io::prelude::*;

extern crate png;
use png::*;

fn main(){
  loop{
    let comando = std::process::Command::new("raspistill").args([
      "-t","5"
    ]).args([
      "-w","720",
    ]).args([
      "-h","480",
    ]).args([
      "-o",
      "imagen.jpg",
    ]).output().unwrap();

    println!("COMANDO: {:?}",comando);
    
    let mut archivo = [0;0];
    std::fs::File::open(std::path::Path::new("imagen.jpg")).unwrap().read(&mut archivo).unwrap();

    let mut cliente = ClientBuilder::new("ws://192.168.100.10:3000/ws").unwrap().connect_insecure().unwrap();
    cliente.send_message(&websocket::Message::text(base64::encode(&archivo))).unwrap();
  }
}
