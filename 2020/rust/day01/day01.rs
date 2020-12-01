fn main() {
    let report = vec![
        1721,
        979,
        366,
        299,
        675,
        1456,
        // Added because otherwise solution is immediately found:
        1,
        2,
        3,
        4,
        5,
        1800,
        1801,
        1802,
    ];
    for front in report.iter() {
        for back in report.iter() {
            if front + back == 2020 {
                println!("{} {} {}", front, back, front * back);
            }
        }
    }
    println!("Hello World!");
}
