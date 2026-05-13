use crate::estructuras::pila::Pila;
use crate::estructuras::cola::Cola;
use crate::helpers::logger::Logger;
use crate::helpers::file_writer::write_file;

fn main() {
    Logger::log("Inicio del programa");

    let mut pila = Pila::new();
    pila.push(10);
    pila.push(20);

    let mut cola = Cola::new();
    cola.enqueue(1);
    cola.enqueue(2);

    write_file("test.txt", "Hola desde Rust");

    println!("Pila pop: {:?}", pila.pop());
    println!("Cola dequeue: {:?}", cola.dequeue());
}