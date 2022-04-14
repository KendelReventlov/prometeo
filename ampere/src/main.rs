extern crate websocket;
extern crate image_base64;
extern crate rascam;

use websocket::ClientBuilder;
use std::process::Command;
use rascam::*;
use std::{thread,time};

fn main(){

  let info = info().unwrap();
  if info.cameras.len() < 1 {
    println!("No se encontraron camaras!")
    std::process::exit(0);
  }

  let mut camara = SimpleCamera::new(info.cameras[0]).unwrap();
  camara.activate().unwrap();
  thread::sleep(time::Duration::from_secs(2));  

  loop{
    let mut cliente = ClientBuilder::new("ws://192.168.100.10:3000/ws").unwrap().connect_insecure().unwrap();
    cliente.send_message(&websocket::Message::text("hola")).unwrap();
    let b = camara.take_one().unwrap();
    println!("BUFFER: {:?}",b);
    println!();
  }
}
