extern crate websocket;
extern crate image_base64;
use websocket::ClientBuilder;
use std::process::Command;
use std::net::TcpStream;
use std::io::prelude::*;

extern crate png;
use png::*;

fn main(){

  std::thread::spawn(move ||{
    let comando = std::process::Command::new("raspivid").args([
        "-t","0"
    ]).arg("-l").args([
      "-w","720",
    ]).args([
      "-h","480",
    ]).args([
        "-o",
        "tcp://127.0.0.1:7878",
    ]).spawn().unwrap();
});

  fn conectar() -> TcpStream{
    match TcpStream::connect("127.0.0.1:7878"){
      Ok(s)=>{return s}
      Err(_)=>{conectar()}
    }
  }
  let mut stream = conectar();


  
  loop{
    let mut buffer = [0;1382400];
    stream.read(&mut buffer).unwrap();
    println!("BUFFER: {:?}",buffer);
    println!("LEN BUFFER: {}",buffer.len());

    let w = std::io::BufWriter::new(std::fs::File::create(std::path::Path::new("imagen.png")).unwrap());
    let mut encoder = png::Encoder::new(w, 720, 480);
    encoder.set_color(png::ColorType::Rgba);

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&buffer).unwrap();


    let mut cliente = ClientBuilder::new("ws://192.168.100.10:3000/ws").unwrap().connect_insecure().unwrap();
    let imagen = image_base64::to_base64("imagen.png");
    cliente.send_message(&websocket::Message::text(imagen)).unwrap();
  }
}
