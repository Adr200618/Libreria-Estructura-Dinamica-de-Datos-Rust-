use std::fs;

pub fn leer_archivo(ruta: &str) -> String {
    fs::read_to_string(ruta).expect("No se pudo leer archivo")
}