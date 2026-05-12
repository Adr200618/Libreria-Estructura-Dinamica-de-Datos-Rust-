use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
pub struct Cola<T> {
    elementos: VecDeque<T>,
}

impl<T> Cola<T> {
    pub fn new() -> Self {
        Cola {
            elementos: VecDeque::new(),
        }
    }

    pub fn encolar(&mut self, valor: T) {
        self.elementos.push_back(valor);
    }

    pub fn desencolar(&mut self) -> Option<T> {
        self.elementos.pop_front()
    }

    pub fn frente(&self) -> Option<&T> {
        self.elementos.front()
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

    pub fn iter(&self) -> std::collections::vec_deque::Iter<'_, T> {
        self.elementos.iter()
    }
}

impl<T> Default for Cola<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> From<Vec<T>> for Cola<T> {
    fn from(vec: Vec<T>) -> Self {
        let mut cola = Cola::new();
        for item in vec {
            cola.encolar(item);
        }
        cola
    }
}