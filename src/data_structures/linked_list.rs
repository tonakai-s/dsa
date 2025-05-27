pub mod singly_linked_list {
    use std::fmt::Display;

    #[derive(Debug)]
    pub struct LinkedList<T> {
        head: Box<Node<T>>,
    }

    #[derive(Debug)]
    pub struct Node<T> {
        data: T,
        next: Option<Box<Node<T>>>,
    }

    impl<T> Node<T> {
        pub fn new(val: T) -> Self {
            Self {
                data: val,
                next: None,
            }
        }
    }

    impl<T: Sized + Copy + Display> LinkedList<T> {
        pub fn new(val: T) -> Self {
            let node = Box::new(Node {
                data: val,
                next: None,
            });
            Self { head: node }
        }

        pub fn size(&self) -> usize {
            let mut size: usize = 1;

            let mut curr = &self.head;
            loop {
                if curr.next.is_none() {
                    break size;
                }

                curr = curr.next.as_ref().unwrap();
                size += 1;
            }
        }

        pub fn insert_end(&mut self, val: T) {
            let node = Box::new(Node::new(val));
            if self.head.next.is_none() {
                self.head.next = Some(node);
                return;
            }

            let mut curr = &mut self.head;
            loop {
                if curr.next.is_none() {
                    break;
                }

                curr = curr.next.as_mut().unwrap();
            }

            curr.next = Some(node);
        }

        pub fn insert_start(&mut self, val: T) {
            // New node
            let node = Box::new(Node::new(val));
            // New node becomes the new head, and return the prev head
            let old_h = std::mem::replace(&mut self.head, node);

            // New head next node is the old head
            self.head.next = Some(old_h);
        }
    }
}
