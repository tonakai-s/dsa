use dsa::data_structures::linked_list::singly_linked_list;

#[test]
fn get_size() {
    let mut list = singly_linked_list::LinkedList::new();
    list.insert_end(3);
    list.insert_end(99);

    let size = list.size();
    assert_eq!(size, 2);

    list.insert_start(25);
    let size = list.size();
    assert_eq!(size, 3);
}

#[test]
fn get_nth() {
    let mut list = singly_linked_list::LinkedList::new();
    list.insert_end(3);
    list.insert_end(99);
    list.insert_end(88);
    list.insert_end(90);
    list.insert_end(12);

    let nth = list.get_nth(0);
    assert!(nth.is_some());
    assert_eq!(*nth.unwrap(), 3);

    let nth = list.get_nth(2);
    assert!(nth.is_some());
    assert_eq!(*nth.unwrap(), 88);
}

#[test]
fn delete_nth() {
    let mut list = singly_linked_list::LinkedList::new();
    list.insert_end(3);
    list.insert_end(99);
    list.insert_end(88);
    list.insert_end(90);
    list.insert_end(12);

    let nth = list.get_nth(2);
    assert_eq!(*nth.unwrap(), 88);

    let _ = list.delete_nth(2);
    let nth = list.get_nth(2);
    assert_eq!(*nth.unwrap(), 90);

    let _ = list.delete_nth(2);
    let nth = list.get_nth(2);
    assert_eq!(*nth.unwrap(), 12);
}

#[test]
fn delete_end() {
    let mut list = singly_linked_list::LinkedList::new();
    list.insert_end(3);
    list.insert_end(99);
    list.insert_end(88);
    list.insert_end(90);
    list.insert_end(12);
    assert_eq!(list.size(), 5);

    let nth = list.get_nth(4);
    assert!(nth.is_some());
    assert_eq!(*nth.unwrap(), 12);

    let _ = list.delete_end();
    assert_eq!(list.size(), 4);

    let nth = list.get_nth(4);
    assert!(nth.is_none());

    let nth = list.get_nth(3);
    assert!(nth.is_some());
    assert_eq!(*nth.unwrap(), 90);
}