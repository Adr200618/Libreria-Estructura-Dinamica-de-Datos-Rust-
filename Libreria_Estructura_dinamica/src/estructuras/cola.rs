pub struct Cola {
    pub data: Vec<i32>,
}

impl Cola {
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    pub fn enqueue(&mut self, v: i32) {
        self.data.push(v);
    }

    pub fn dequeue(&mut self) -> Option<i32> {
        if self.data.is_empty() {
            None
        } else {
            Some(self.data.remove(0))
        }
    }
}