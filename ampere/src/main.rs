extern crate websocket;
extern crate image_base64;
use websocket::ClientBuilder;
use std::process::Command;

fn main(){

  let output = Command::new("python").arg("camera.py");
  std::thread::sleep_ms(5000);

  loop{
    let base = image_base64::to_base64("imagen.png");
    let mut cliente = ClientBuilder::new("ws://192.168.100.10:3000/ws").unwrap().connect_insecure().unwrap();
    cliente.send_message(&websocket::Message::text(base)).unwrap();
    std::thread::sleep_ms(2500);
  }
}
