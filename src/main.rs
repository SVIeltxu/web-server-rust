mod servidor;

use std::net::{TcpListener};
use std::thread;

fn main() {
    println!("Programa en ejecucion!!");

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); // Abre el puerto en nuesto ordenador

    for stream in listener.incoming() {
        // Se crea un bucle para acceptar peticiones

        match stream {
            // Nos da un mensaje depende si se a establecido o no la conexion con el user
            Ok(conexion) => {
                thread::spawn(move || {
                    servidor::manejar_conexion(conexion);
                });
            }

            Err(e) => {
                println!("Fallo en la conexión: {e}");
            }
        }
    }
}
