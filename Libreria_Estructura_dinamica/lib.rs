// Modulo principal de la libreria
pub mod estructuras;
pub mod helpers;

// Re-exportar estructuras principales para facilitar el uso
pub use estructuras::{Arbol, Cola, Pila, Grafo};
pub use helpers::{read_file, write_file, Logger, LogLevel};