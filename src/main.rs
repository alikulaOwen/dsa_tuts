// filepath: /dsa_tuts/dsa_tuts/src/main.rs
mod linked_list;

use linked_list::LinkedList;

fn main() {
    let mut list: LinkedList<i32> = LinkedList::new();

    list.push(1);
    list.push(2);
    list.push(3);

    println!("{:?}", list.peek().unwrap_or(&-1));

    println!("{:?}", list.pop().unwrap_or(-1));

    list.peek_mut().map(|value| *value = 42);

    println!("{:?}", list.peek().unwrap_or(&-1));

}