pub mod static_stack {
    #[derive(Debug)]
    pub enum StaticStackError {
        StackOverflow,
        StackUnderflow,
        StackEmpty
    }

    #[derive(Debug)]
    pub struct StaticStack<T> {
        elements: Vec<T>
    }

    impl<T: Sized + Copy> StaticStack<T> {
        pub fn new(size: isize) -> Result<Self, &'static str> {
            if size < 1 {
                return Err("Size cannot be less than 1");
            }
            Ok(
                Self {
                    elements: Vec::with_capacity(size.try_into().unwrap())
                }
            )
        }

        pub fn push(&mut self, element: T) -> Result<(), StaticStackError> {
            if self.is_full() {
                return Err(StaticStackError::StackOverflow);
            }

            self.elements.push(element);
            Ok(())
        }

        pub fn pop(&mut self) -> Result<T, StaticStackError> {
            if self.is_empty() {
                return Err(StaticStackError::StackUnderflow);
            }

            Ok(self.elements.pop().unwrap())
        }

        pub fn peek(&self) -> Option<&T> {
            self.elements.last()
        }

        pub fn is_empty(&self) -> bool {
            self.elements.len() == 0
        }

        pub fn is_full(&self) -> bool {
            self.elements.len() == self.elements.capacity()
        }

        pub fn size(&self) -> usize {
            self.elements.len()
        }
    }
}