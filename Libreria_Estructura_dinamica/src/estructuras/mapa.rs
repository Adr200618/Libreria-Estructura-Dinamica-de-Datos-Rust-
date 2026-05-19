use std::fs;

pub struct Entrada {
    clave: String,
    valor: String,
}

pub struct Mapa {
    datos: Vec<Entrada>,
}

impl Mapa {
    pub fn new() -> Self {
        Self { datos: vec![] }
    }

    pub fn insertar(&mut self, clave: String, valor: String) {
        self.datos.push(Entrada { clave, valor });
    }

    pub fn buscar(&self, clave: &str) {
        for entrada in &self.datos {
            if entrada.clave == clave {
                println!("{} -> {}", entrada.clave, entrada.valor);
                return;
            }
        }

        println!("No encontrado");
    }

    pub fn eliminar(&mut self, clave: &str) {
        self.datos.retain(|e| e.clave != clave);
    }

    pub fn mostrar(&self) {
        println!("\nMAPA:");

        for entrada in &self.datos {
            println!("{} -> {}", entrada.clave, entrada.valor);
        }
    }

    pub fn cargar_desde_archivo(&mut self, ruta: &str) {
        let contenido = fs::read_to_string(ruta).expect("No se pudo leer archivo");

        for linea in contenido.lines() {
            let partes: Vec<&str> = linea.split(',').collect();

            if partes.len() == 2 {
                self.insertar(
                    partes[0].trim().to_string(),
                    partes[1].trim().to_string(),
                );
            }
        }
    }
}