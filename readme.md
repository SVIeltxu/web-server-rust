# Servidor HTTP en Rust desde Cero

Un servidor web HTTP concurrente, modular y ligero construido completamente en **Rust** sin utilizar frameworks externos (`std`). Este proyecto ha sido desarrollado como práctica de aprendizaje para comprender los fundamentos de los sockets TCP, el manejo de archivos, la concurrencia multihilo y las pruebas unitarias.

---

## 🚀 Características Principales

* **Servidor TCP Nativo:** Utiliza `TcpListener` y `TcpStream` de la librería estándar de Rust.
* **Concurrencia Real:** Implementa gestión multihilo con `std::thread::spawn` y bloques `move` para atender múltiples peticiones de clientes simultáneamente sin bloqueos.
* **Manejo Dinámico de Archivos:** Lee recursos del sistema de archivos local (`index.html`, etc.) de forma segura.
* **Cabeceras HTTP Dinámicas (`Content-Type`):** Detecta automáticamente la extensión del archivo solicitado (como `.html`, `.css`, `.png`, `.jpg`) para enviar el tipo MIME correcto al navegador.
* **Página de Error Personalizada:** Devuelve una respuesta HTTP `404 Not Found` utilizando una página de error dedicada si el recurso no existe.
* **Arquitectura Modular:** El código está desacoplado, separando la lógica del servidor en módulos (`servidor.rs`) y la ejecución principal (`main.rs`).
* **Pruebas Automatizadas:** Incluye tests unitarios integrados para verificar el correcto funcionamiento de las funciones críticas.

---

## 📂 Estructura del Proyecto

```text
gestion-archivos/
├── src/
│   ├── main.rs          # Punto de entrada y bucle de escucha del servidor
│   └── servidor.rs      # Lógica de conexiones, rutas y tipos de contenido
├── index.html           # Página principal del sitio web
├── error404.html        # Página mostrada en caso de error 404
├── Cargo.toml           # Configuración del proyecto y dependencias
└── README.md            # Documentación del proyecto