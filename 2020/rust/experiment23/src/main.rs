#![feature(linked_list_cursors)]

use std::collections::LinkedList;
use std::collections::linked_list::CursorMut;

impl CursorMut<'_, i32> {
    pub fn hello() {
        println!("hello");
    }
}

fn main() {
    println!("Hello, world!");


    let mut list: LinkedList<u32> = LinkedList::new();

    list.push_back(0);
    list.push_back(1);
    list.push_back(2);

    let mut cursor = list.cursor_back_mut();
    println!("Cursor1a {:?}", cursor.current());
    // println!("Cursor1 {:?} {:?} {:?}", cursor.peek_prev(), cursor.index(), cursor.peek_next());
    println!("Cursor1f {:?}", cursor);
    cursor.move_prev();
    println!("Cursor2a {:?}", cursor.current());
    // println!("Cursor2 {:?} {:?} {:?}", cursor.peek_prev(), cursor.index(), cursor.peek_next());
    println!("Cursor2f {:?}", cursor);

    // let cursor2 = cursor.clone();
    // let cup_current_ll = list.pop_front().unwrap();
    // list.push_back(5);
    // let mut list = list;
    println!("Cursor3a {:?}", &cursor.current());

    // println!("Cursor3 {:?} {:?} {:?}", cursor.peek_prev(), cursor.index(), cursor.peek_next());
    // println!("Cursor3f {:?}", cursor.as_cursor());

    // list.push_back(cup_current_ll);
    // println!("Cursor4 {:?} {:?} {:?}", cursor.peek_prev(), cursor.index(), cursor.peek_next());

}
