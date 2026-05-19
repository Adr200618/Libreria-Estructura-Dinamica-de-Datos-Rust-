mod estructuras;
mod helpers;
mod traits;
mod enums;
use estructuras::{
arbol::Arbol,
pila::Pila,
cola::Cola,
mapa::Mapa,
};
use std::io;
fn main() {
let mut arbol = Arbol::new();
let mut pila = Pila::new();
let mut cola = Cola::new();
let mut mapa = Mapa::new();
loop {
println!("\n===== LIBRERIA DE ESTRUCTURAS =====");
println!("1. Arbol");
println!("2. Pila");
println!("3. Cola");
println!("4. Mapa");
println!("0. Salir");
let mut opcion = String::new();
io::stdin().read_line(&mut opcion).unwrap();
match opcion.trim() {
"1" => {
arbol.cargar_desde_archivo("datos_arbol.txt");

arbol.mostrar();
arbol.dfs();
arbol.bfs();
}
"2" => {
pila.cargar_desde_archivo("datos_pila.txt");
pila.mostrar();
}
"3" => {
cola.cargar_desde_archivo("datos_cola.txt");
cola.mostrar();
}
"4" => {
mapa.cargar_desde_archivo("datos_mapas.txt");
mapa.mostrar();
}
"0" => break,
_ => println!("Opcion invalida"),
}
}
}