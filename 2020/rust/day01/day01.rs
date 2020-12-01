fn parse_report(report: &[i32]) -> (i32, i32) {
    let mut front = 1111;
    let mut back = 2222;
    for tfront in report.iter() {
        for tback in report.iter() {
            if tfront + tback == 2020 {
                front = *tfront;
                back = *tback;
                break;
            }
        }
        if tfront + back == 2020 {
            break;
        }
    }
    (front, back)
}

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
    let (front, back) = parse_report(&report);
    println!("{} {} {} {}", front, back, front + back, front * back);
}
