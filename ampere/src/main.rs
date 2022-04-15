extern crate websocket;
extern crate image_base64;
use websocket::ClientBuilder;
use std::process::Command;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

fn handle_connection(mut stream:TcpStream){
  let mut buffer = [0;1024];

  stream.read(&mut buffer).unwrap();
  let response = "HTTP/1.1 200 OK\r\n\r\n";
  stream.write(response.as_bytes()).unwrap();
  stream.flush().unwrap();
}

fn main(){
  Command::new("raspistill");
  let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
  for stream in listener.incoming(){
    let stream = stream.unwrap();

    handle_connection(stream);
  }
  /*loop{
    let mut cliente = ClientBuilder::new("ws://192.168.100.10:3000/ws").unwrap().connect_insecure().unwrap();
    cliente.send_message(&websocket::Message::text("hola")).unwrap();
  }*/
}
