mod estructuras;
mod helpers;

use crate::estructuras::pila::Pila;
use crate::estructuras::cola::Cola;
use crate::estructuras::arbol::Nodo;
use crate::estructuras::grafo::Mapa;

use crate::helpers::logger::Logger;
use crate::helpers::file_writer::write_file;

fn main() {

    Logger::log("Inicio del programa");

    // PILA
    let mut pila = Pila::new();

    pila.push(10);
    pila.push(20);

    // COLA
    let mut cola = Cola::new();

    cola.enqueue(1);
    cola.enqueue(2);

    // ARBOL

    let arbol = Nodo::new(50);

    // MAPA

    let mut mapa = Mapa::new();

    mapa.insert("Edad".to_string(), 20);
    mapa.insert("Semestre".to_string(), 5);

    // ARCHIVOS

    write_file("test.txt", "Hola desde Rust");

    // RESULTADOS

    println!("Pila pop: {:?}", pila.pop());

    println!("Cola dequeue: {:?}", cola.dequeue());

    println!("Nodo del arbol: {}", arbol.valor);

    println!("Contenido del mapa: {:?}", mapa.data);
}