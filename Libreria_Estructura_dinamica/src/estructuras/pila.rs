#[derive(Debug, Clone, PartialEq)]
pub struct Pila<T> {
    elementos: Vec<T>,
}

impl<T> Pila<T> {
    pub fn new() -> Self {
        Pila {
            elementos: Vec::new(),
        }
    }

    pub fn apilar(&mut self, valor: T) {
        self.elementos.push(valor);
    }

    pub fn desapilar(&mut self) -> Option<T> {
        self.elementos.pop()
    }

    pub fn cima(&self) -> Option<&T> {
        self.elementos.last()
    }

    pub fn esta_vacia(&self) -> bool {
        self.elementos.is_empty()
    }

    pub fn tamanio(&self) -> usize {
        self.elementos.len()
    }

    pub fn vaciar(&mut self) {
        self.elementos.clear();
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.elementos.iter()
    }
}

impl<T> Default for Pila<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> From<Vec<T>> for Pila<T> {
    fn from(vec: Vec<T>) -> Self {
        let mut pila = Pila::new();
        for item in vec {
            pila.apilar(item);
        }
        pila
    }
}