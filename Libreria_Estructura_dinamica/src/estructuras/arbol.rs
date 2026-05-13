pub struct Nodo {
    pub valor: i32,
    pub left: Option<Box<Nodo>>,
    pub right: Option<Box<Nodo>>,
}

impl Nodo {
    pub fn new(valor: i32) -> Self {
        Self {
            valor,
            left: None,
            right: None,
        }
    }
}