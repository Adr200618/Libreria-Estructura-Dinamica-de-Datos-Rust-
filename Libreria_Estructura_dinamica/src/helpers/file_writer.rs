use std::fs::OpenOptions;
use std::io::Write;

pub fn escribir_archivo(ruta: &str, contenido: &str) {
    let mut archivo = OpenOptions::new()
        .create(true)
        .append(true)
        .open(ruta)
        .unwrap();

    writeln!(archivo, "{}", contenido).unwrap();
}