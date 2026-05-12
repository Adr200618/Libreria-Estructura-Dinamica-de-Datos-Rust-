use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq)]
pub struct Nodo<T> {
    pub valor: T,
    pub hijos: Vec<Nodo<T>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Arbol<T> {
    pub raiz: Option<Box<Nodo<T>>>,
}

impl<T: Clone + Debug + PartialEq> Arbol<T> {
    pub fn new() -> Self {
        Arbol { raiz: None }
    }

    pub fn con_raiz(valor: T) -> Self {
        Arbol {
            raiz: Some(Box::new(Nodo {
                valor,
                hijos: Vec::new(),
            })),
        }
    }

    pub fn agregar_hijo(&mut self, padre_valor: &T, valor_hijo: T) -> bool {
        if let Some(raiz) = &mut self.raiz {
            Self::agregar_hijo_recursivo(raiz, padre_valor, valor_hijo)
        } else {
            false
        }
    }

    fn agregar_hijo_recursivo(nodo: &mut Box<Nodo<T>>, padre_valor: &T, valor_hijo: T) -> bool {
        if nodo.valor == *padre_valor {
            nodo.hijos.push(Nodo {
                valor: valor_hijo,
                hijos: Vec::new(),
            });
            return true;
        }

        for hijo in &mut nodo.hijos {
            if Self::agregar_hijo_recursivo(&mut Box::new(hijo.clone()), padre_valor, valor_hijo.clone()) {
                return true;
            }
        }
        false
    }

    pub fn recorrido_preorden(&self) -> Vec<T> {
        let mut resultado = Vec::new();
        if let Some(raiz) = &self.raiz {
            Self::preorden_recursivo(raiz, &mut resultado);
        }
        resultado
    }

    fn preorden_recursivo(nodo: &Box<Nodo<T>>, resultado: &mut Vec<T>) {
        resultado.push(nodo.valor.clone());
        for hijo in &nodo.hijos {
            Self::preorden_recursivo(&Box::new(hijo.clone()), resultado);
        }
    }

    pub fn altura(&self) -> usize {
        if let Some(raiz) = &self.raiz {
            Self::altura_recursivo(raiz)
        } else {
            0
        }
    }

    fn altura_recursivo(nodo: &Box<Nodo<T>>) -> usize {
        if nodo.hijos.is_empty() {
            1
        } else {
            1 + nodo.hijos.iter()
                .map(|h| Self::altura_recursivo(&Box::new(h.clone())))
                .max()
                .unwrap_or(0)
        }
    }

    pub fn tamanio(&self) -> usize {
        if let Some(raiz) = &self.raiz {
            Self::tamanio_recursivo(raiz)
        } else {
            0
        }
    }

    fn tamanio_recursivo(nodo: &Box<Nodo<T>>) -> usize {
        1 + nodo.hijos.iter()
            .map(|h| Self::tamanio_recursivo(&Box::new(h.clone())))
            .sum::<usize>()
    }
}

impl<T> Default for Arbol<T> where T: Clone + Debug + PartialEq {
    fn default() -> Self {
        Self::new()
    }
}