fn main() {
    let numbers = vec![2, 3, 4, 5, 6];
    let are_large = numbers.iter().map(|x| x > &2).collect::<Vec<_>>();
    println!("Which are large? {:?}", are_large);
    // let all_are_large = are_large.all();
    //                               ^^^ method not found in `std::vec::Vec<bool>`
    // let all_are_large = are_large.iter().all();
    //                                      ^^^- supplied 0 arguments
    //                                      |
    //                                      expected 1 argument
    // let all_are_large = are_large.iter().all(true);
    //                                          ^^^^ expected an `FnMut<(&bool,)>` closure, found `bool
    let all_are_large = are_large.iter().all(|x| x == &true);
    println!("Are all large? {:?}", all_are_large);
}