extern crate websocket;
extern crate image_base64;
extern crate opencv;

use websocket::ClientBuilder;
use std::process::Command;
use opencv::{
  highgui,
  prelude::*,
  Result,
  videoio,
};

fn main(){
  let ventana = "video capture";
  highgui::named_window(ventana, highgui::WINDOW_AUTOSIZE).unwrap();
  #[cfg(ocvrs_opencv_branch_32)]
  let mut cam = videoio::VideoCapture::new_default(0).unwrap();
  #[cfg(not(ocvrs_opencv_branch_32))]
  let mut cam = videoio::VideoCapture::new(0,videoio::CAP_ANY).unwrap();
  let opened = videoio::VideoCapture::is_opened(&cam).unwrap();
  if !opened{
    println!("Hubo un error");
    std::process::exit(0);
  } 

  loop{
    let mut cliente = ClientBuilder::new("ws://192.168.100.10:3000/ws").unwrap().connect_insecure().unwrap();
    cliente.send_message(&websocket::Message::text("hola")).unwrap();
    let mut frame = Mat::default();
    cam.read(&mut frame).unwrap();
    if frame.size().unwrap().width > 0{
      highgui::imshow(window, &mut frame).unwrap();
    }
    let key = highgui::wait_key(10).unwrap();
    if key > 0 && key != 255{
      break;
    }
  }
}
