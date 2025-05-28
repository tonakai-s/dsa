pub mod static_queue {
    use std::collections::VecDeque;

    #[derive(Debug)]
    pub enum StaticQueueError {
        QueueOverflow,
        QueueUnderflow,
    }

    // Dequeue <- Head <- [1, 2, 3, 4] <- Tail <- Enqueue
    #[derive(Debug)]
    pub struct StaticQueue<T> {
        queue: VecDeque<T>,
    }

    impl<T: Sized + Copy> StaticQueue<T> {
        pub fn new(size: usize) -> Self {
            Self {
                queue: VecDeque::with_capacity(size),
            }
        }

        pub fn enqueue(&mut self, el: T) -> Result<(), StaticQueueError> {
            if self.is_full() {
                return Err(StaticQueueError::QueueOverflow);
            }

            self.queue.push_front(el);
            Ok(())
        }

        pub fn dequeue(&mut self) -> Result<T, StaticQueueError> {
            if self.is_empty() {
                return Err(StaticQueueError::QueueUnderflow);
            }

            Ok(self.queue.pop_back().unwrap())
        }

        pub fn head(&self) -> Option<&T> {
            if self.is_empty() {
                return None;
            }

            self.queue.front()
        }

        pub fn tail(&self) -> Option<&T> {
            if self.is_empty() {
                return None;
            }

            self.queue.back()
        }

        pub fn is_full(&self) -> bool {
            self.queue.len() == self.queue.capacity()
        }

        pub fn is_empty(&self) -> bool {
            self.queue.len() == 0
        }

        pub fn size(&self) -> usize {
            self.queue.len()
        }
    }
}
