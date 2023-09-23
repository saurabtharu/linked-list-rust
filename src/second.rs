// AN OK SINGLY-lINKED STACK

use std::mem;
// Making generic `List`
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    data: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self { head: Link::None }
    }

    pub fn push(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            // next: mem::replace(&mut self.head, Link::None),
            next: self.head.take(),
        });
        self.head = Link::Some(new_node)
    }

    pub fn pop(&mut self) -> Option<T> {
        // match mem::replace(&mut self.head, Link::None) {
        /*
        match self.head.take() {
            Link::None => None,
            Link::Some(node) => {
                self.head = node.next;
                Some(node.data)
            }
        }
        */
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head
            // .as_ref()
            .as_mut()
            .map(|node| &mut node.data)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        // let mut cur_link = mem::replace(&mut self.head, Link::None);
        let mut cur_link = self.head.take();

        while let Link::Some(mut boxed_node) = cur_link {
            // cur_link = mem::replace(&mut boxed_node.next, Link::None);
            cur_link = boxed_node.next.take();
        }
    }
}

pub fn first_main() {}

mod test {
    use super::List;
    #[test]
    fn basics_test() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);
        list.push(5);
        list.push(7);

        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.pop(), Some(5));

        list.push(42);
        list.push(43);

        assert_eq!(list.pop(), Some(43));
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn peek_test() {
        let mut list = List::new();

        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);

        list.push(42);
        list.push(43);
        list.push(44);

        assert_eq!(list.peek(), Some(&44));
        assert_eq!(list.peek_mut(), Some(&mut 44));

        list.peek_mut().map(|value| *value = 12);

        assert_eq!(list.peek(), Some(&12));
        assert_eq!(list.pop(), Some(12));
    }
}
