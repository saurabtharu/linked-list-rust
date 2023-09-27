// AN OK SINGLY-lINKED STACK

/*

use std::mem;

*/
// Making generic `List`
#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Link<T>,
}

#[derive(Debug)]
pub struct IntoIter<T>(List<T>);

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
        // '---------''------' '--------------------'
        //     node      &             &node.data
        //     it returns &node.data if it exits
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.data)
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
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

pub fn second_main() {
    let mut list = List::new();
    list.push(4);
    list.push(6);
    list.push(3);
    list.push(2);

    let mut iter = list.into_iter();
    for _ in 0..7 {
        println!("{:?}", &iter.next());
    }
}

mod test {
    use crate::second::List;
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

    #[test]
    fn into_iter_test() {
        let mut list = List::new();
        list.push(1);
        list.push(3);
        list.push(4);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }
}
