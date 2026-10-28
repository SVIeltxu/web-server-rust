use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

fn leerArchivo(peticion: Vec<&str>) -> Option<String> {
    match fs::read_to_string(&peticion[1][1..]) {
        Ok(contenido) => {
            println!("¡Archivo {} leído con éxito!", peticion[1]);
            Some(contenido)
        }
        Err(e) => {
            println!("Error: No se pudo encontrar o leer el archivo ({e})");
            None
        }
    }
}

fn main() {
    println!("Hello, world!");

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); // Abre el puerto en nuesto ordenador

    for stream in listener.incoming() {
        // Se crea un bucle para acceptar peticiones

        match stream {
            // Nos da un mensaje depende si se a establecido o no la conexion con el user
            Ok(mut conexion) => {
                // 1. Capturamos el TcpStream en la variable 'conexion'
                // 2. Le preguntamos su dirección IP usando el método .peer_addr()
                match conexion.peer_addr() {
                    Ok(addr) => println!("Nuevo cliente conectado desde la IP: {addr}"),
                    Err(e) => println!("No se pudo obtener la IP: {e}"),
                }

                let mut buf_reader = BufReader::new(&mut conexion);

                if let Some(resultado_linea) = buf_reader.lines().next() {
                    match resultado_linea {
                        Ok(primera_linea) => {
                            let archivo = primera_linea.split_whitespace().collect::<Vec<&str>>();
                            println!("Petición recibida: {}", archivo[1]);

                            if let Some(contenido) = leerArchivo(archivo) {
                                let respuesta = format!(
                                    "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n{}",
                                    contenido
                                );
                                conexion.write_all(respuesta.as_bytes()).unwrap();
                            } else {
                                println!("No se pudo enviar el archivo porque no existe.");
                            }
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