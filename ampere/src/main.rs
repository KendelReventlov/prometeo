extern crate websocket;
extern crate image_base64;
extern crate rpi_video_rs;

use websocket::ClientBuilder;
use std::process::Command;
use rpi_video_rs::recorder::Recorder;

fn main(){ 

  let mut recorder = Recorder::new(None);

  loop{
    let mut cliente = ClientBuilder::new("ws://192.168.100.10:3000/ws").unwrap().connect_insecure().unwrap();
    cliente.send_message(&websocket::Message::text("hola")).unwrap();
    match recorder.run(){
      Ok(res)=>{
        println!("Video guardado en el path: ",res.output_file_path);
      }
      Err(_)=>{
        println!("Ocurrió un error!");
      }
    }
    
  }
}
