// AN OK SINGLY-lINKED STACK

use std::mem;
pub struct List {
    head: Link,
}

type Link = Option<Box<Node>>;

struct Node {
    data: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        Self { head: Link::None }
    }

    pub fn push(&mut self, data: i32) {
        let new_node = Box::new(Node {
            data,
            // next: mem::replace(&mut self.head, Link::None),
            next: self.head.take(),
        });
        self.head = Link::Some(new_node)
    }

    pub fn pop(&mut self) -> Option<i32> {
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
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = /*mem::replace(&mut self.head, Link::None);*/ self.head.take();

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
}
