use std::fs;

fn parse_report(report: &[i32], sum: i32) -> (i32, i32) {
    let mut front = 1111;
    let mut back = 2222;
    for tfront in report.iter() {
        for tback in report.iter() {
            if tfront + tback == sum {
                front = *tfront;
                back = *tback;
                break;
            }
        }
        if tfront + back == sum {
            break;
        }
    }
    (front, back)
}

fn parse_report_three(report: &[i32]) {
    let head = &report[0];
    let tail = &report[1..];
    let (front, back) = parse_report(tail, 2020 - head);
    if head + front + back == 2020 {
        println!("Found: {} {} {} {} {}", head, front, back, head + front + back, head * front * back);
    } else {
        parse_report_three(tail);
    }
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
    let (front, back) = parse_report(&report, 2020);
    println!("{} {} {} {}", front, back, front + back, front * back);
    parse_report_three(&report);

    // let report2: Vec<&str> = fs::read_to_string("input.txt").expect("Something went wrong reading the file.").split("\n").collect();
    let report1 = fs::read_to_string("input.txt").expect("Something went wrong reading the file.");
    let report2: Vec<&str> = report1.split("\n").collect();
    let report = report2.iter().filter(|x| **x != "").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
    let (front, back) = parse_report(&report, 2020);
    println!("{} {} {} {}", front, back, front + back, front * back);
    parse_report_three(&report);
}
