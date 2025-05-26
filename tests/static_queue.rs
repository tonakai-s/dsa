use dsa::data_structures::StaticQueue;

#[test]
fn enqueue() {
    let mut queue = StaticQueue::new(4);
    assert_eq!(queue.size(), 0);

    let _ = queue.enqueue(2);
    assert_eq!(queue.size(), 1);

    let _ = queue.enqueue(3);
    assert_eq!(queue.size(), 2);
}

#[test]
fn dequeue() {
    let mut queue = StaticQueue::new(4);
    assert_eq!(queue.size(), 0);

    let _ = queue.enqueue(2);
    assert_eq!(queue.size(), 1);

    let head = queue.dequeue().unwrap();
    assert_eq!(queue.size(), 0);
    assert_eq!(head, 2);
}

#[test]
fn head() {
    let mut queue = StaticQueue::new(4);
    assert_eq!(queue.size(), 0);
    assert!(queue.head().is_none());

    let _ = queue.enqueue(2);
    assert_eq!(queue.size(), 1);

    let _ = queue.enqueue(3);
    assert_eq!(queue.size(), 2);

    let head_ref = queue.head();
    assert!(head_ref.is_some());
    assert_eq!(head_ref.unwrap(), &3);

    assert_eq!(queue.size(), 2);
}

#[test]
fn tail() {
    let mut queue = StaticQueue::new(4);
    assert_eq!(queue.size(), 0);
    assert!(queue.tail().is_none());

    let _ = queue.enqueue(2);
    assert_eq!(queue.size(), 1);

    let _ = queue.enqueue(3);
    assert_eq!(queue.size(), 2);

    let tail_ref = queue.tail();
    assert!(tail_ref.is_some());
    assert_eq!(tail_ref.unwrap(), &2);

    assert_eq!(queue.size(), 2);
}