pub fn iniciar_tcp_camara(){
    std::process::Command::new("raspivid").args([
        "-t","0"
    ]).arg("-l").args([
        "-o",
        "tcp://127.0.0.1:7878",
    ]);    
}
