extern crate websocket;
extern crate image;
extern crate image_base64;
use websocket::ClientBuilder;

fn main(){
  loop{
    let base = image_base64::to_base64("imagen.jpg");
    let mut cliente = ClientBuilder::new("ws://192.168.100.10:3000/ws").unwrap().connect_insecure().unwrap();
    cliente.send_message(&websocket::Message::text(base)).unwrap();
    std::thread::sleep_ms(5000);
  }
}
