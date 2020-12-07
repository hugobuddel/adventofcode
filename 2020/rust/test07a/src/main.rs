
fn main () {
    println!("Advent of Code 2020 Day 7.");

    let line = "abcd efgh";
    // let mut mylines: Vec<&str> = Vec::new();
    let mut mylines: Vec<String> = Vec::new();
    for word in line.split(" ") {
        let word2 = word.replace("a", "b");
        // mylines.push(&word2);
        mylines.push(word2);
    }
    println!("{:?}", mylines);
}

// error[E0597]: `word2` does not live long enough
//   --> src/main.rs:9:22
//    |
// 9  |         mylines.push(&word2);
//    |                      ^^^^^^ borrowed value does not live long enough
// 10 |     }
//    |     - `word2` dropped here while still borrowed
// 11 |     println!("{:?}", mylines);
//    |                      ------- borrow later used here
//
// error: aborting due to previous error
//
// For more information about this error, try `rustc --explain E0597`.
// error: could not compile `test07a`.
