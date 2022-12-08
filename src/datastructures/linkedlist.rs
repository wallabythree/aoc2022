pub mod single {
    struct Node<T> {
        elem: T,
        next: Option<Box<Node<T>>>,
    }

    pub struct List<T> {
        head: Option<Box<Node<T>>>
    }
}

