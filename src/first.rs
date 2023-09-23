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
    // SO the main IDEA is:
    //      so 'self.head' should point to the newly pushed node and the newly pushed node should
    //      point to element pointed by 'self.head'
    pub fn push(&mut self, data: i32) {
        /*
            |^^^^^^^^^^^^^^^^^^^^^^^^^|
            | Node.data = data        |
            | Node.data = Link::Empty |
            |.........................|
                Box::new()
        */
        let new_node = Box::new(Node {
            data,
            // here 'self.head' is replaced with 'Link::Empty'
            // and previous value of 'self.head' is returned and assigned to 'next' field
            next: mem::replace(&mut self.head, Link::Empty),
        });

        /*
         *  here 'self.head' is made to point to the newly pushed node
                                    _                                 _
                                    |     |^^^^^^^^^^^^^^^^^^^^^^^^^|   |
            self.head -> Link::More |     | Node.data = data        |   |
                                    |     | Node.data = Link::Empty |   |
                                    |     |.........................|   |
                                    |..                               ..|
        */
        self.head = Link::More(new_node)
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
    /*
        MAIN IDEA:
        ========
            -> make 'self.head' Link::Empty so that it cannot point to anyone and return the value of 'self.head' using `mem::replace`
                - NOTE: here returned value will be either `Link::Empty` or `Link::More(Box<Node>)`

            -> if `mem::replace` returns `Link::Empty` then return `Option::None`
            -> if `mem::replace` returns `Link::More(Box<Node>)` then handle accordingly
                Hanlding `Link::More(Box<Node>)`
                        -> self.head = node.next
                        -> return node.data
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

mod test {
    // making new module requires importing the required module explicitly
    use super::List;
    // use crate::first::List;
    #[test]
    fn basics_test() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);
        list.push(5);
        list.push(7);

        // Check the normal removal
        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.pop(), Some(5));

        // Push some more just to make sure nothing's corrupted
        list.push(42);
        list.push(43);

        // Check the normal removal
        assert_eq!(list.pop(), Some(43));
        assert_eq!(list.pop(), Some(42));

        // Check exhaustion
        // assert_eq!(list.pop(), Some(1));
        // assert_eq!(list.pop(), None);
    }
}
