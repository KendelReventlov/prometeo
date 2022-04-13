extern crate websocket;
extern crate image_base64;
extern crate rscam;
use websocket::ClientBuilder;
use std::process::Command;

fn main(){

  let mut camera = rscam::new("/dev/video0").unwrap();

  camera.start(&rscam::Config{
    interval: (1,30),
    resolution: (1080,720),
    format: b"MJPG",
    ..Default::default()
  }).unwrap();

  loop{
    let cuadro = camera.capture().unwrap();
    println!("{}",cuadro);
    let mut cliente = ClientBuilder::new("ws://192.168.100.10:3000/ws").unwrap().connect_insecure().unwrap();
    cliente.send_message(&websocket::Message::text("hola")).unwrap();
    std::thread::sleep_ms(5000);
  }
}
