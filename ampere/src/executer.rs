pub fn iniciar_tcp_camara(){
    std::thread::spawn(move ||{
        let comando = std::process::Command::new("raspivid").args([
            "-t","0"
        ]).arg("-l").args([
            "-o",
            "tcp://127.0.0.1:7878",
        ]).status().unwrap();
        println!("COMANDO: {:?}",comando);
            
    });
}
