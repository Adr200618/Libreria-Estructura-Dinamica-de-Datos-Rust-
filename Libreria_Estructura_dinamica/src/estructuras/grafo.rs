use std::collections::HashMap;

pub struct Mapa {
    pub data: HashMap<String, i32>,
}

impl Mapa {
    pub fn new() -> Self {
        Self { data: HashMap::new() }
    }

    pub fn insert(&mut self, k: String, v: i32) {
        self.data.insert(k, v);
    }
}