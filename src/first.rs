// pub says we want people outside this module to be able to use List

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

pub fn first_main() {
    // println!("{:?}", list);
}
