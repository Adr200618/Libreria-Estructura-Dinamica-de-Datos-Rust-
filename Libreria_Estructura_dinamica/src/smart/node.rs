use std::rc::Rc;

pub struct Node<T> {
    pub value: T,
    pub next: Option<Rc<Node<T>>>,
}