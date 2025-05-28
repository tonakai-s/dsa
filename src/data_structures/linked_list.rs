pub mod singly_linked_list {
    use std::fmt::Display;

    #[derive(Debug)]
    /// Front -> head -> 1 -> 2 -> 3 -> tail -> End
    pub struct LinkedList<T> {
        head: Option<Box<Node<T>>>,
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

    impl<T: Sized + Copy + Display + PartialEq> LinkedList<T> {
        pub fn new() -> Self {
            Self { head: None }
        }

        pub fn size(&self) -> usize {
            if self.head.is_none() {
                return 0;
            }

            let mut size: usize = 1;

            let mut curr = self.head.as_ref().unwrap();

            while curr.next.is_some() {
                size += 1;
                curr = curr.next.as_ref().unwrap();
            }

            size
        }

        pub fn insert_end(&mut self, val: T) {
            let node = Box::new(Node::new(val));

            if self.head.is_none() {
                self.head = Some(node);
                return;
            }

            let mut curr = self.head.as_mut().unwrap();
            while curr.next.is_some() {
                curr = curr.next.as_mut().unwrap();
            }

            curr.next = Some(node);
        }

        pub fn insert_start(&mut self, val: T) {
            let head = self.head.as_mut().unwrap();
            // New node
            let node = Box::new(Node::new(val));
            // New node becomes the new head, and return the prev head
            let old_h = std::mem::replace(head, node);

            // New head next node is the old head
            head.next = Some(old_h);
        }

        /// Get the value of the NTH node
        pub fn get_nth(&self, idx: usize) -> Option<&T> {
            self.head.as_ref()?;

            let mut curr = self.head.as_ref().unwrap();
            for _ in std::iter::repeat(()).take(idx) {
                curr.next.as_ref()?;
                curr = curr.next.as_ref().unwrap();
            }

            Some(&curr.data)
        }

        pub fn delete_nth(&mut self, idx: usize) -> bool {
            if self.head.is_none() {
                return false;
            }

            let mut curr = self.head.as_mut().unwrap();
            for _ in std::iter::repeat(()).take(idx - 1) {
                if curr.next.is_none() {
                    return false;
                }

                curr = curr.next.as_mut().unwrap();
            }

            curr.next = curr.next.as_mut().unwrap().next.take();
            true
        }

        pub fn delete_end(&mut self) {
            if self.head.is_none() {
                return;
            }

            let mut curr = self.head.as_mut().unwrap();
            // We need the second_last node to invalidate the 'next' field
            while curr.next.is_some() && curr.next.as_ref().unwrap().next.is_some() {
                curr = curr.next.as_mut().unwrap();
            }

            curr.next = None;
        }

        pub fn exists(&self, val: T) -> bool {
            if self.head.is_none() {
                return false;
            }

            let mut curr = self.head.as_ref().unwrap();
            while curr.next.is_some() {
                if curr.data == val {
                    return true;
                }
                curr = curr.next.as_ref().unwrap();
            }

            false
        }
    }
}
