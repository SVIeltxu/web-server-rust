use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::fs;

pub fn manejar_conexion(mut conexion: TcpStream) {
    match conexion.peer_addr() {
        Ok(addr) => println!("Nuevo cliente conectado desde la IP: {addr}"),
        Err(e) => println!("No se pudo obtener la IP: {e}"),
    }

    let buf_reader = BufReader::new(&mut conexion);

    if let Some(resultado_linea) = buf_reader.lines().next() {
        match resultado_linea {
            Ok(primera_linea) => {
                let archivo = primera_linea.split_whitespace().collect::<Vec<&str>>();
                println!("Petición recibida: {}", archivo[1]);

                if let Some(contenido) = leer_archivo(archivo[1]) {
                    let respuesta = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: {}\r\n\r\n{}",
                        obtener_content_type(archivo[1]),
                        contenido
                    );
                    conexion.write_all(respuesta.as_bytes()).unwrap();
                } else {
                    println!("No se pudo enviar el archivo porque no existe.");
                    let error_cont: String = leer_archivo("error404.html").unwrap();
                    let respuesta = format!(
                        "HTTP/1.1 404 NOT FOUND\r\nContent-Type: {}\r\n\r\n{}",
                        obtener_content_type(archivo[1]),
                        error_cont
                    );
                    conexion.write_all(respuesta.as_bytes()).unwrap();
                }
            }
            Err(e) => {
                println!("Error al leer la línea: {}", e);
            }
        }
    }
}


fn leer_archivo(peticion: &str) -> Option<String> {
    let ruta: &str = if peticion == "/" {
        "index.html"
    } else if peticion.starts_with('/') {
        &peticion[1..]
    } else {
        peticion
    };

    match fs::read_to_string(ruta) {
        Ok(contenido) => {
            println!("¡Archivo {} leído con éxito!", ruta);
            Some(contenido)
        }
        Err(e) => {
            println!("Error: No se pudo encontrar o leer el archivo ({e})");
            None
        }
    }
}

fn obtener_content_type(ruta: &str) -> &str {
    if ruta.ends_with(".css") {
        "text/css"
    } else if ruta.ends_with(".png") {
        "image/png"
    } else if ruta.ends_with(".jpg") || ruta.ends_with(".jpeg") {
        "image/jpeg"
    } else {
        "text/html" // Por defecto para html u otros
    }
}


#[cfg(test)]
mod tests {
    use super::*; // Importa todo lo de fuera del módulo (tus funciones)

    #[test]
    fn test_obtener_content_type() {
        // Comprobamos que devuelva el tipo MIME correcto según la extensión
        assert_eq!(obtener_content_type("estilos.css"), "text/css");
        assert_eq!(obtener_content_type("imagen.png"), "image/png");
        assert_eq!(obtener_content_type("foto.jpg"), "image/jpeg");
        assert_eq!(obtener_content_type("foto.jpeg"), "image/jpeg");
        assert_eq!(obtener_content_type("index.html"), "text/html");
        assert_eq!(obtener_content_type("cualquier_cosa.txt"), "text/html");
    }

    #[test]
    fn test_leer_archivo_inexistente() {
        // Comprobamos que si le pasamos un archivo que no existe, devuelva None en lugar de romper
        let resultado = leer_archivo("archivo_que_no_existe_12345.html");
        assert!(resultado.is_none());
    }
}