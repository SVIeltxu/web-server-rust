use std::io::{BufRead, BufReader};
use std::net::TcpListener;

fn main() {
    println!("Hello, world!");

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); // Abre el puerto en nuesto ordenador

    for stream in listener.incoming() {
        // Se crea un bucle para acceptar peticiones

        match stream {
            // Nos da un mensaje depende si se a establecido o no la conexion con el user
            Ok(conexion) => {
                // 1. Capturamos el TcpStream en la variable 'conexion'
                // 2. Le preguntamos su dirección IP usando el método .peer_addr()
                match conexion.peer_addr() {
                    Ok(addr) => println!("Nuevo cliente conectado desde la IP: {addr}"),
                    Err(e) => println!("No se pudo obtener la IP: {e}"),
                }

                let buf_reader = BufReader::new(&conexion);

                if let Some(resultado_linea) = buf_reader.lines().next() {
                    match resultado_linea {
                        Ok(primera_linea) => {
                            println!("Petición recibida: {}", primera_linea);
                        }
                        Err(e) => {
                            println!("Error al leer la línea: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                println!("Fallo en la conexión: {e}");
            }
        }
    }
}
