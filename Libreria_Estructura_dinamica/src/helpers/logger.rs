use crate::helpers::file_writer::escribir_archivo;

pub fn log(mensaje: &str) {
    escribir_archivo("logs/app.log", mensaje);
}