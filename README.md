# Libreria de Estructuras Dinámicas en Rust

Este repositorio contiene una biblioteca Rust para trabajar con estructuras de datos dinámicas y ejemplos de uso.

## Descripción

El proyecto está diseñado para apoyar el aprendizaje y la implementación de estructuras de datos dinámicas en Rust.
Incluye ejemplos básicos de:

- Pila (`Pila`)
- Cola (`Cola`)
- Árbol simple (`Nodo`)
- Grafo/Mapa sencillo (`Mapa`)
- Lectura y escritura de archivos
- Logger básico para seguimiento de ejecución

## Estructura del proyecto

- `Libreria_Estructura_dinamica/cargo.toml` - paquete principal de Rust
- `Libreria_Estructura_dinamica/src/main.rs` - programa de ejemplo que usa las estructuras y helpers
- `Libreria_Estructura_dinamica/src/lib.rs` - exporta los módulos públicos
- `Libreria_Estructura_dinamica/src/estructuras/` - módulos de estructuras de datos
- `Libreria_Estructura_dinamica/src/helpers/` - herramientas de apoyo como logger y archivo

## Uso

Desde la carpeta del proyecto principal:

```powershell
cd Libreria_Estructura_dinamica
cargo run
```

O desde la raíz del repositorio:

```powershell
cargo run --manifest-path Libreria_Estructura_dinamica/cargo.toml
```

El programa de ejemplo realiza lo siguiente:

- Crea y manipula una pila
- Crea y manipula una cola
- Crea un nodo de árbol
- Inserta datos en un mapa simple
- Escribe texto en `test.txt`
- Muestra resultados por consola

## Archivos principales

- `src/main.rs` - ejecuta el flujo de ejemplo
- `src/estructuras/pila.rs` - implementación de pila
- `src/estructuras/cola.rs` - implementación de cola
- `src/estructuras/arbol.rs` - implementación básica de árbol
- `src/estructuras/grafo.rs` - implementación básica de mapa/grafo
- `src/helpers/logger.rs` - logger simple
- `src/helpers/file_writer.rs` - escribe en archivos

## Mejora sugerida

- Agregar pruebas unitarias con `cargo test`
- Extender la implementación de árbol y grafo con más operaciones
- Añadir manejo de errores más robusto en las operaciones de archivo

## Notas

El archivo `test.txt` se crea o se actualiza en la raíz del paquete `Libreria_Estructura_dinamica`.

