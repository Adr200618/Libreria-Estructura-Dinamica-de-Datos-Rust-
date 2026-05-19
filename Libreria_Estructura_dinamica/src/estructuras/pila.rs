use std::fs;

pub struct Pila {
    datos: Vec<String>,
}

impl Pila {
    pub fn new() -> Self {
        Self { datos: vec![] }
    }

    pub fn push(&mut self, valor: String) {
        self.datos.push(valor);
    }

    pub fn pop(&mut self) {
        self.datos.pop();
    }

    pub fn buscar(&self, valor: &str) -> bool {
        self.datos.contains(&valor.to_string())
    }

    pub fn mostrar(&self) {
        println!("\nPILA:");

        for dato in self.datos.iter().rev() {
            println!("{}", dato);
        }
    }

    pub fn cargar_desde_archivo(&mut self, ruta: &str) {
        let contenido = fs::read_to_string(ruta).expect("No se pudo leer archivo");

        for linea in contenido.lines() {
            self.push(linea.to_string());
        }
    }
}