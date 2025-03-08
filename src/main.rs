struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>
}

struct LinkedList<T> {
    head: Option<Box<Node<T>>>
    
}

fn main() {
    let list: LinkedList<i32> = LinkedList { head: None };
}

