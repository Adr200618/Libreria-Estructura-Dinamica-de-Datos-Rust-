use std::fs;

pub struct Cola {
    datos: Vec<String>,
}

impl Cola {
    pub fn new() -> Self {
        Self { datos: vec![] }
    }

    pub fn enqueue(&mut self, valor: String) {
        self.datos.push(valor);
    }

    pub fn dequeue(&mut self) {
        if !self.datos.is_empty() {
            self.datos.remove(0);
        }
    }

    pub fn buscar(&self, valor: &str) -> bool {
        self.datos.contains(&valor.to_string())
    }

    pub fn mostrar(&self) {
        println!("\nCOLA:");

        for dato in &self.datos {
            println!("{}", dato);
        }
    }

    pub fn cargar_desde_archivo(&mut self, ruta: &str) {
        let contenido = fs::read_to_string(ruta).expect("No se pudo leer archivo");

        for linea in contenido.lines() {
            self.enqueue(linea.to_string());
        }
    }
}
