pub mod linked_list;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn works_normal_list() {
        let mut list = linked_list::LinkedList::new();

        list.append(1);
        list.append(2);
        list.append(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
    }

    #[test]
    fn works_normal_list_peeking() {
        let mut list = linked_list::LinkedList::new();

        assert_eq!(list.peek(), None);
        list.append(1);
        list.append(2);
        list.append(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.pop(), Some(3));

        list.mut_peek().map(|value| { *value = 67 });

        assert_eq!(list.peek(), Some(&67));
        assert_eq!(list.mut_peek().take(), Some(&mut 67));
    }
    #[test]
    fn works_storing_strings() {
        let mut list = linked_list::LinkedList::new();

        list.append("Hello".to_string());
        list.append("World".to_string());

        assert_eq!(list.pop(), Some("World".to_string()));
        assert_eq!(list.pop(), Some("Hello".to_string()));
    }
}