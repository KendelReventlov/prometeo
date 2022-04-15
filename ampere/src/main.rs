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
      "-t",
      "150",
    ]).argas([
      "-w",
      "720",
    ]).argas([
      "-h",
      "480",
    ]).args([
      "-o",
      "imagen.jpg",
    ]).status().unwrap();

    println!("SALIDA: {:?}",comando);

    let mut archivo = Vec::new();
    std::fs::File::open(std::path::Path::new("imagen.jpg")).unwrap().read_to_end(&mut archivo).unwrap();
    println!("LEN FILE: {:?}",archivo.len());

    let mut cliente = ClientBuilder::new("ws://192.168.100.10:3000/ws").unwrap().connect_insecure().unwrap();
    cliente.send_message(&websocket::Message::text(base64::encode(&archivo))).unwrap();
    println!("Se envió exitosamente el mensaje!");
  }
}
