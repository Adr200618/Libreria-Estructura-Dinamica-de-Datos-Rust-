use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq)]
pub struct Grafo<T> {
    adyacencias: HashMap<T, HashSet<T>>,
}

impl<T: std::hash::Hash + Eq + Clone + Debug> Grafo<T> {
    pub fn new() -> Self {
        Grafo {
            adyacencias: HashMap::new(),
        }
    }

    pub fn agregar_vertice(&mut self, vertice: T) {
        self.adyacencias.entry(vertice).or_insert_with(HashSet::new);
    }

    pub fn agregar_arista(&mut self, origen: T, destino: T) {
        self.adyacencias.entry(origen.clone()).or_insert_with(HashSet::new).insert(destino.clone());
        self.adyacencias.entry(destino).or_insert_with(HashSet::new);
    }

    pub fn agregar_arista_dirigida(&mut self, origen: T, destino: T) {
        self.adyacencias.entry(origen.clone()).or_insert_with(HashSet::new).insert(destino);
        self.adyacencias.entry(origen).or_insert_with(HashSet::new);
    }

    pub fn obtener_vecinos(&self, vertice: &T) -> Option<&HashSet<T>> {
        self.adyacencias.get(vertice)
    }

    pub fn contiene_vertice(&self, vertice: &T) -> bool {
        self.adyacencias.contains_key(vertice)
    }

    pub fn bfs(&self, inicio: T) -> Vec<T> {
        let mut visitados = HashSet::new();
        let mut cola = VecDeque::new();
        let mut resultado = Vec::new();

        cola.push_back(inicio.clone());
        visitados.insert(inicio);

        while let Some(actual) = cola.pop_front() {
            resultado.push(actual.clone());
            
            if let Some(vecinos) = self.adyacencias.get(&actual) {
                for vecino in vecinos {
                    if !visitados.contains(vecino) {
                        visitados.insert(vecino.clone());
                        cola.push_back(vecino.clone());
                    }
                }
            }
        }

        resultado
    }

    pub fn dfs(&self, inicio: T) -> Vec<T> {
        let mut visitados = HashSet::new();
        let mut resultado = Vec::new();
        self.dfs_recursivo(&inicio, &mut visitados, &mut resultado);
        resultado
    }

    fn dfs_recursivo(&self, actual: &T, visitados: &mut HashSet<T>, resultado: &mut Vec<T>) {
        if visitados.contains(actual) {
            return;
        }

        visitados.insert(actual.clone());
        resultado.push(actual.clone());

        if let Some(vecinos) = self.adyacencias.get(actual) {
            for vecino in vecinos {
                self.dfs_recursivo(vecino, visitados, resultado);
            }
        }
    }

    pub fn vertices(&self) -> Vec<T> {
        self.adyacencias.keys().cloned().collect()
    }

    pub fn tamanio(&self) -> usize {
        self.adyacencias.len()
    }

    pub fn esta_vacio(&self) -> bool {
        self.adyacencias.is_empty()
    }
}

impl<T: std::hash::Hash + Eq + Clone + Debug> Default for Grafo<T> {
    fn default() -> Self {
        Self::new()
    }
}