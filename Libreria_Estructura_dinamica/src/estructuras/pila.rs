pub struct Pila {
    pub data: Vec<i32>,
}

impl Pila {
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    pub fn push(&mut self, v: i32) {
        self.data.push(v);
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.data.pop()
    }
}