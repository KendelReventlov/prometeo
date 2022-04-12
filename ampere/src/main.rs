extern crate websocket;
extern crate image_base64;
extern crate rscam;
use websocket::ClientBuilder;

fn main(){
  let mut camara = rscam::new("dev/video0").unwrap();
  camara.start(&rscam::Config{
    interval:(1,30),
    resolution:(720,480),
    format:b"MJPG",
    ..Default::default()
  });

  loop{
    let frame = camara.capture().unwrap();
    let mut archivo = std::fs::File::create("imagen.png").unwrap();
    archivo.write_all(&frame[..]).unwrap();
    let base = image_base64::to_base64("imagen.png");
    let mut cliente = ClientBuilder::new("ws://192.168.100.10:3000/ws").unwrap().connect_insecure().unwrap();
    cliente.send_message(&websocket::Message::text(base)).unwrap();
    std::thread::sleep_ms(5000);
  }
}
