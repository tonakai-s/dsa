pub mod data_structures;

#[cfg(test)]
mod tests {
    use super::data_structures::StaticStack;

    #[test]
    fn push() {
        let mut stack = StaticStack::new(2).unwrap();
        let _ = stack.push(2);
        let last_push = stack.push(2);

        assert_eq!(stack.size(), 2);
        assert!(last_push.is_ok());
        dbg!(&stack);
    }

    #[test]
    fn push_overflow() {
        let mut stack = StaticStack::new(2).unwrap();
        let _ = stack.push(2);
        let _ = stack.push(2);
        assert_eq!(stack.size(), 2);

        let result = stack.push(2);
        assert_eq!(stack.size(), 2);
        dbg!(&stack);

        assert!(result.is_err());
    }

    #[test]
    fn pop() {
        let mut stack = StaticStack::new(2).unwrap();
        let _ = stack.push(2);
        let _ = stack.push(2);

        dbg!(&stack);
        assert_eq!(stack.size(), 2);

        let popped = stack.pop();
        assert!(popped.is_ok());
        dbg!(&popped);

        assert_eq!(stack.size(), 1);
        dbg!(&stack);
    }

    #[test]
    fn pop_underflow() {
        let mut stack: StaticStack<i32> = StaticStack::new(2).unwrap();
        assert_eq!(stack.size(), 0);

        let popped = stack.pop();
        assert!(popped.is_err());
        dbg!(&popped);

        assert_eq!(stack.size(), 0);
        dbg!(&stack);
    }

    #[test]
    fn peek() {
        let mut stack: StaticStack<i32> = StaticStack::new(2).unwrap();
        let _ = stack.push(3);
        assert_eq!(stack.size(), 1);

        let peeked = stack.peek();
        assert!(peeked.is_some());
        dbg!(&peeked);
        assert_eq!(stack.size(), 1);

        let _ = stack.pop();
        let peeked = stack.peek();
        assert!(peeked.is_none());
        dbg!(&peeked);
        assert_eq!(stack.size(), 0);
    }
}
