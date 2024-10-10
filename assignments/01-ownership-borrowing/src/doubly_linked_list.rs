// Now try and implement a doubly linked version. Give an explanation
// for why this doesn't work.

struct Node {
    val: i32,
    next: Link,
    prev: Link,
}

type Link = Option<Box<Node>>;

pub struct LinkedStack {
    head: Link,
    // tail: Link,
}

impl LinkedStack {
    fn new() -> Self {
        Self { head: None }
    }

    fn push(&mut self, val: i32) {
        let node = Box::new(Node {
            val,
            next: None,
            prev: self.head.take(),
        });

        self.head = Some(node);
    }

    fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|mut old_head| {
            self.head = old_head.prev.take();
            old_head.val
        })
    }
}