use mi_libreria::{
    Arbol, Cola, Pila, Grafo,
    read_file, write_file, Logger, LogLevel
};

fn main() {
    // Inicializar logger
    let logger = Logger::new()
        .with_file("aplicacion.log")
        .with_levels(vec![LogLevel::Info, LogLevel::Warning, LogLevel::Error, LogLevel::Debug]);

    logger.info("Iniciando aplicación de prueba");

    // Prueba de Árbol
    prueba_arbol(&logger);
    
    // Prueba de Cola
    prueba_cola(&logger);
    
    // Prueba de Pila
    prueba_pila(&logger);
    
    // Prueba de Grafo
    prueba_grafo(&logger);
    
    // Prueba de File Handler
    prueba_file_handler(&logger);
    
    logger.success("Todas las pruebas completadas exitosamente");
}

fn prueba_arbol(logger: &Logger) {
    println!("\n=== PRUEBA DE ÁRBOL ===");
    
    let mut arbol = Arbol::con_raiz("Raíz".to_string());
    arbol.agregar_hijo(&"Raíz".to_string(), "Hijo 1".to_string());
    arbol.agregar_hijo(&"Raíz".to_string(), "Hijo 2".to_string());
    arbol.agregar_hijo(&"Hijo 1".to_string(), "Nieto 1".to_string());
    
    logger.info("Árbol creado exitosamente");
    
    println!("Recorrido preorden: {:?}", arbol.recorrido_preorden());
    println!("Altura del árbol: {}", arbol.altura());
    println!("Tamaño del árbol: {}", arbol.tamanio());
    println!("Árbol: {:#?}", arbol);
}

fn prueba_cola(logger: &Logger) {
    println!("\n=== PRUEBA DE COLA ===");
    
    let mut cola = Cola::new();
    cola.encolar(10);
    cola.encolar(20);
    cola.encolar(30);
    
    logger.debug(format!("Cola creada con {} elementos", cola.tamanio()).as_str());
    
    println!("Frente de la cola: {:?}", cola.frente());
    println!("Elemento desencolado: {:?}", cola.desencolar());
    println!("Tamaño después de desencolar: {}", cola.tamanio());
    println!("Elementos restantes: {:?}", cola.iter().collect::<Vec<_>>());
}

fn prueba_pila(logger: &Logger) {
    println!("\n=== PRUEBA DE PILA ===");
    
    let mut pila = Pila::new();
    pila.apilar("Primero");
    pila.apilar("Segundo");
    pila.apilar("Tercero");
    
    logger.debug(format!("Pila creada con {} elementos", pila.tamanio()).as_str());
    
    println!("Cima de la pila: {:?}", pila.cima());
    println!("Elemento desapilado: {:?}", pila.desapilar());
    println!("Tamaño después de desapilar: {}", pila.tamanio());
    println!("Elementos restantes: {:?}", pila.iter().collect::<Vec<_>>());
}

fn prueba_grafo(logger: &Logger) {
    println!("\n=== PRUEBA DE GRAFO ===");
    
    let mut grafo = Grafo::new();
    
    // Agregar vértices
    grafo.agregar_vertice("A");
    grafo.agregar_vertice("B");
    grafo.agregar_vertice("C");
    grafo.agregar_vertice("D");
    
    // Agregar aristas
    grafo.agregar_arista("A", "B");
    grafo.agregar_arista("A", "C");
    grafo.agregar_arista("B", "D");
    grafo.agregar_arista("C", "D");
    
    logger.info("Grafo creado exitosamente");
    
    println!("Vértices del grafo: {:?}", grafo.vertices());
    println!("Vecinos de 'A': {:?}", grafo.obtener_vecinos(&"A"));
    println!("BFS desde 'A': {:?}", grafo.bfs("A"));
    println!("DFS desde 'A': {:?}", grafo.dfs("A"));
    println!("Tamaño del grafo: {}", grafo.tamanio());
}

fn prueba_file_handler(logger: &Logger) {
    println!("\n=== PRUEBA DE FILE HANDLER ===");
    
    let archivo_prueba = "prueba.txt";
    let contenido = "Este es un archivo de prueba\nLínea 2\nLínea 3";
    
    match write_file(archivo_prueba, contenido) {
        Ok(_) => logger.info("Archivo escrito exitosamente"),
        Err(e) => logger.error(&format!("Error al escribir archivo: {}", e)),
    }
    
    match read_file(archivo_prueba) {
        Ok(contenido_leido) => {
            logger.info("Archivo leído exitosamente");
            println!("Contenido del archivo:\n{}", contenido_leido);
        }
        Err(e) => logger.error(&format!("Error al leer archivo: {}", e)),
    }
    
    // Agregar contenido
    match mi_libreria::helpers::append_to_file(archivo_prueba, "Nueva línea agregada") {
        Ok(_) => logger.info("Contenido agregado exitosamente"),
        Err(e) => logger.error(&format!("Error al agregar contenido: {}", e)),
    }
    
    // Limpiar archivo de prueba
    let _ = std::fs::remove_file(archivo_prueba);
}