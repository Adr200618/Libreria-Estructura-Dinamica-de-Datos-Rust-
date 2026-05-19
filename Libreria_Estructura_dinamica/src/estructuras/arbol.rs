use std::cell::RefCell;
use std::fs;
use std::rc::Rc;

pub type NodoRef = Rc<RefCell<Nodo>>;

#[derive(Debug)]
pub struct Nodo {
    pub valor: String,
    pub hijos: Vec<NodoRef>,
}

pub struct Arbol {
    pub raiz: Option<NodoRef>,
}

impl Arbol {
    pub fn new() -> Self {
        Self { raiz: None }
    }

    pub fn insertar_raiz(&mut self, valor: String) {
        let nodo = Rc::new(RefCell::new(Nodo {
            valor,
            hijos: vec![],
        }));

        self.raiz = Some(nodo);
    }

    pub fn insertar_hijo(&self, padre: &str, hijo: String) {
        if let Some(raiz) = &self.raiz {
            Self::insertar_recursivo(raiz.clone(), padre, hijo);
        }
    }

    fn insertar_recursivo(actual: NodoRef, padre: &str, hijo: String) {
        if actual.borrow().valor == padre {
            actual.borrow_mut().hijos.push(Rc::new(RefCell::new(Nodo {
                valor: hijo,
                hijos: vec![],
            })));
            return;
        }

        let hijos = actual.borrow().hijos.clone();

        for h in hijos {
            Self::insertar_recursivo(h, padre, hijo.clone());
        }
    }

    pub fn mostrar(&self) {
        if let Some(raiz) = &self.raiz {
            Self::mostrar_recursivo(raiz.clone(), 0);
        }
    }

    fn mostrar_recursivo(nodo: NodoRef, nivel: usize) {
        for _ in 0..nivel {
            print!("  ");
        }

        println!("{}", nodo.borrow().valor);

        let hijos = nodo.borrow().hijos.clone();

        for hijo in hijos {
            Self::mostrar_recursivo(hijo, nivel + 1);
        }
    }

    pub fn dfs(&self) {
        println!("\nDFS:");

        if let Some(raiz) = &self.raiz {
            Self::dfs_recursivo(raiz.clone());
        }

        println!();
    }

    fn dfs_recursivo(nodo: NodoRef) {
        print!("{} ", nodo.borrow().valor);

        let hijos = nodo.borrow().hijos.clone();

        for hijo in hijos {
            Self::dfs_recursivo(hijo);
        }
    }

    pub fn bfs(&self) {
        println!("\nBFS:");

        if self.raiz.is_none() {
            return;
        }

        let mut cola = vec![self.raiz.as_ref().unwrap().clone()];

        while !cola.is_empty() {
            let actual = cola.remove(0);

            print!("{} ", actual.borrow().valor);

            let hijos = actual.borrow().hijos.clone();

            for hijo in hijos {
                cola.push(hijo);
            }
        }

        println!();
    }

    pub fn cargar_desde_archivo(&mut self, ruta: &str) {
        let contenido = fs::read_to_string(ruta).expect("No se pudo leer archivo");

        for linea in contenido.lines() {
            let partes: Vec<&str> = linea.split(',').collect();

            if partes.len() == 2 {
                let padre = partes[0].trim();
                let hijo = partes[1].trim().to_string();

                if self.raiz.is_none() {
                    self.insertar_raiz(padre.to_string());
                }

                self.insertar_hijo(padre, hijo);
            }
        }
    }
}
