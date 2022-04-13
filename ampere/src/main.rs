extern crate websocket;
extern crate image_base64;
extern crate camera_capture;
use websocket::ClientBuilder;
use std::process::Command;
pub enum camara{
  encendida(camera_capture::Builder),
  apagada,
}

fn main(){

  loop{
    let cam= camera_capture::create(0).unwrap();
    for imagen in cam.fps(5.0).unwrap().start().unwrap(){
      println!("{:?}",imagen);
    }

    let mut cliente = ClientBuilder::new("ws://192.168.100.10:3000/ws").unwrap().connect_insecure().unwrap();
    cliente.send_message(&websocket::Message::text("hola")).unwrap();
    std::thread::sleep_ms(5000);
  }
}
