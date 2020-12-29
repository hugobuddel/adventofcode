#![feature(linked_list_cursors)]

fn main() {
    println!("Hello, world!");

    use std::collections::LinkedList;

    let mut list: LinkedList<u32> = LinkedList::new();

    list.push_back(0);
    list.push_back(1);
    list.push_back(2);

    let mut cursor = list.cursor_back();
    println!("Cursor1 {:?} {:?} {:?}", cursor.peek_prev(), cursor.index(), cursor.peek_next());
    cursor.move_prev();
    println!("Cursor2 {:?} {:?} {:?}", cursor.peek_prev(), cursor.index(), cursor.peek_next());
}
