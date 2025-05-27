use dsa::data_structures::linked_list::singly_linked_list;

#[test]
fn get_size() {
    let mut list = singly_linked_list::LinkedList::new(6);
    list.insert_end(3);
    list.insert_end(99);

    let size = list.size();
    assert_eq!(size, 3);

    list.insert_start(25);
    let size = list.size();
    assert_eq!(size, 4);
}
