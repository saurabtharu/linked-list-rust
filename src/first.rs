/*
            LAYOUT is like
        --------------------
    if `List` is empty:
                [ptr] -> Link::Empty


    if `List` contain some element
    [ptr: Link::More(Box<Node>)] -> [Node.data, Node.next] -> [Node.data, Node.next] -> [Node.data, Node.next] -> [Node.data, Node.next] -> [Node.data, Node.next] -> Link::Empty

*/

use std::mem;
// pub says we want people outside this module to be able to use List
pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    data: i32,
    next: Link,
}

// now we need to create all the function for `List`
impl List {
    /// Self is an alias for the type at the top next to impl.
    pub fn new() -> Self {
        // at very the begining `List` is empty
        // so initialized with `Link::Empty` value
        Self { head: Link::Empty }
    }

    // `push` mutates the list so parameter 'self' is `mut`
    // function `push` adds new node at the begining of the List
    pub fn push(&mut self, data: i32) {
        // TODO
        let new_code = Box::new(Node {
            data,
            // so self.head should point to the newly pushed node and the newly pushed node should
            // point to the Link::Empty i.e. Null
            next: mem::replace(&mut self.head, Link::Empty),
            // next: Link::Empty,
        });
        self.head = Link::More(new_code)
    }

    // `pop` function also mutates the list sof 'self' is `mut`

    /*
       Check if the list is empty.
       If it's empty, just return None
       If it's not empty
       remove the head of the list
       remove its elem
       replace the list's head with its next
       return Some(elem)

    */
    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.data)
            }
        }
    }
}

pub fn first_main() {
    // println!("{:?}", list);
}
