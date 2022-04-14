extern crate websocket;
extern crate image_base64;

use websocket::ClientBuilder;

fn main(){
  loop{
    let mut cliente = ClientBuilder::new("ws://192.168.100.10:3000/ws").unwrap().connect_insecure().unwrap();
    cliente.send_message(&websocket::Message::text("hola")).unwrap();
  }
}
