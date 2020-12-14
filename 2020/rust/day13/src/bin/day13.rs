//! --- Day 13: Shuttle Search ---
//
// Your ferry can make it safely to a nearby port, but it won't get much
// further. When you call to book another ship, you discover that no ships
// embark from that port to your vacation island. You'll need to get from the
// port to the nearest airport.
//
// Fortunately, a shuttle bus service is available to bring you from the sea
// port to the airport! Each bus has an ID number that also indicates how often
// the bus leaves for the airport.
//
// Bus schedules are defined based on a timestamp that measures the number of
// minutes since some fixed reference point in the past. At timestamp 0, every
// bus simultaneously departed from the sea port. After that, each bus travels
// to the airport, then various other locations, and finally returns to the sea
// port to repeat its journey forever.
//
// The time this loop takes a particular bus is also its ID number: the bus
// with ID 5 departs from the sea port at timestamps 0, 5, 10, 15, and so on.
// The bus with ID 11 departs at 0, 11, 22, 33, and so on. If you are there
// when the bus departs, you can ride that bus to the airport!
//
// Your notes (your puzzle input) consist of two lines. The first line is your
// estimate of the earliest timestamp you could depart on a bus. The second
// line lists the bus IDs that are in service according to the shuttle company;
// entries that show x must be out of service, so you decide to ignore them.
//
// To save time once you arrive, your goal is to figure out the earliest bus
// you can take to the airport. (There will be exactly one such bus.)
//
// For example, suppose you have the following notes:
//
// 939
// 7,13,x,x,59,x,31,19
//
// Here, the earliest timestamp you could depart is 939, and the bus IDs in
// service are 7, 13, 59, 31, and 19. Near timestamp 939, these bus IDs depart
// at the times marked D:
//
// time   bus 7   bus 13  bus 59  bus 31  bus 19
// 929      .       .       .       .       .
// 930      .       .       .       D       .
// 931      D       .       .       .       D
// 932      .       .       .       .       .
// 933      .       .       .       .       .
// 934      .       .       .       .       .
// 935      .       .       .       .       .
// 936      .       D       .       .       .
// 937      .       .       .       .       .
// 938      D       .       .       .       .
// 939      .       .       .       .       .
// 940      .       .       .       .       .
// 941      .       .       .       .       .
// 942      .       .       .       .       .
// 943      .       .       .       .       .
// 944      .       .       D       .       .
// 945      D       .       .       .       .
// 946      .       .       .       .       .
// 947      .       .       .       .       .
// 948      .       .       .       .       .
// 949      .       D       .       .       .
//
// The earliest bus you could take is bus ID 59. It doesn't depart until
// timestamp 944, so you would need to wait 944 - 939 = 5 minutes before it
// departs. Multiplying the bus ID by the number of minutes you'd need to wait
// gives 295.
//
// What is the ID of the earliest bus you can take to the airport multiplied by
// the number of minutes you'll need to wait for that bus?
//
// --- Part Two ---
//
// The shuttle company is running a contest: one gold coin for anyone that can
// find the earliest timestamp such that the first bus ID departs at that time
// and each subsequent listed bus ID departs at that subsequent minute. (The
// first line in your input is no longer relevant.)
//
// For example, suppose you have the same list of bus IDs as above:
//
// 7,13,x,x,59,x,31,19
//
// An x in the schedule means there are no constraints on what bus IDs must
// depart at that time.
//
// This means you are looking for the earliest timestamp (called t) such that:
//
//     Bus ID 7 departs at timestamp t.
//     Bus ID 13 departs one minute after timestamp t.
//     There are no requirements or restrictions on departures at two or three minutes after timestamp t.
//     Bus ID 59 departs four minutes after timestamp t.
//     There are no requirements or restrictions on departures at five minutes after timestamp t.
//     Bus ID 31 departs six minutes after timestamp t.
//     Bus ID 19 departs seven minutes after timestamp t.
//
// The only bus departures that matter are the listed bus IDs at their specific
// offsets from t. Those bus IDs can depart at other times, and other bus IDs
// can depart at those times. For example, in the list above, because bus ID 19
// must depart seven minutes after the timestamp at which bus ID 7 departs, bus
// ID 7 will always also be departing with bus ID 19 at seven minutes after
// timestamp t.
//
// In this example, the earliest timestamp at which this occurs is 1068781:
//
// time     bus 7   bus 13  bus 59  bus 31  bus 19
// 1068773    .       .       .       .       .
// 1068774    D       .       .       .       .
// 1068775    .       .       .       .       .
// 1068776    .       .       .       .       .
// 1068777    .       .       .       .       .
// 1068778    .       .       .       .       .
// 1068779    .       .       .       .       .
// 1068780    .       .       .       .       .
// 1068781    D       .       .       .       .
// 1068782    .       D       .       .       .
// 1068783    .       .       .       .       .
// 1068784    .       .       .       .       .
// 1068785    .       .       D       .       .
// 1068786    .       .       .       .       .
// 1068787    .       .       .       D       .
// 1068788    D       .       .       .       D
// 1068789    .       .       .       .       .
// 1068790    .       .       .       .       .
// 1068791    .       .       .       .       .
// 1068792    .       .       .       .       .
// 1068793    .       .       .       .       .
// 1068794    .       .       .       .       .
// 1068795    D       D       .       .       .
// 1068796    .       .       .       .       .
// 1068797    .       .       .       .       .
//
// In the above example, bus ID 7 departs at timestamp 1068788 (seven minutes
// after t). This is fine; the only requirement on that minute is that bus ID
// 19 departs then, and it does.
//
// Here are some other examples:
//
//     The earliest timestamp that matches the list 17,x,13,19 is 3417.
//     67,7,59,61 first occurs at timestamp 754018.
//     67,x,7,59,61 first occurs at timestamp 779210.
//     67,7,x,59,61 first occurs at timestamp 1261476.
//     1789,37,47,1889 first occurs at timestamp 1202161486.
//
// However, with so many bus IDs in your list, surely the actual earliest
// timestamp will be larger than 100000000000000!
//
// What is the earliest timestamp such that all of the listed bus IDs depart
// at offsets matching their positions in the list?

use std::fs;

extern crate pest;
#[macro_use]
extern crate pest_derive;
use pest::Parser;

#[derive(Parser)]
#[grammar = "busses.pest"]
pub struct BussesParser;

// x % y but returns y if x % y == 0
fn myremainder(x: usize, y: usize) -> usize {
    let r = if y > 0 {
        x % y
    } else {
        x
    };
    match r {
        0 => {y}
        _ => {r}
    }
}

fn check_timestamp(timetest: usize, busses: &Vec<(usize, usize)>) -> bool {
    let timeleft = busses.iter().map(|bus| (bus.0, bus.1, bus.0 - myremainder(timetest, bus.0))).collect::<Vec<_>>();
    let busses_ok = busses.iter().map(|bus| bus.0 - myremainder(timetest, bus.0) == bus.1).collect::<Vec<_>>();
    let all_busses_ok = busses_ok.iter().all(|bus| bus == &true);
    // if all_busses_ok || timetest == 1068781 {
    if all_busses_ok {
        println!("{} {} {:?}", timetest, all_busses_ok, busses_ok);
        println!("{:?}", timeleft);
    }
    all_busses_ok
}

fn main() {
    println!("Advent of Code 2020 Day 13!");

    // let filename = "inputexample.txt";
    let filename = "input.txt";
    let unparsed_file = fs::read_to_string(filename).expect("Error reading file.");

    let busfile = BussesParser::parse(Rule::file, &unparsed_file)
        .expect("Unsuccessful parse")
        .next().unwrap();

    let mut pair = busfile.into_inner();
    let timenow: usize = pair.next().unwrap().as_str().parse().unwrap();
    let busses_o = pair.next().unwrap().into_inner().enumerate()
        .filter(|x| x.1.as_str() != "x")
        .map(|x| (x.1.as_str().parse::<usize>().unwrap(), x.0))
        // .map(|x| (x.1, myremainder(x.0 , x.1)))
        .collect::<Vec<_>>();
    let busses = busses_o.iter()
        // This fixes the (13, 36)
        // .map(|x| (x.0, myremainder(x.1 , x.0)))
        .map(|x| (x.0, x.1 % x.0))
        .collect::<Vec<_>>();
    println!("Time: {}", timenow);
    println!("Busses Old: {:?}", busses_o);
    println!("Busses New: {:?}", busses);

    let mut timeleft = busses.iter().map(|bus| (bus.0 - timenow % bus.0, bus.0)).collect::<Vec<_>>();
    println!("Timeleft: {:?}", timeleft);
    timeleft.sort();
    let (wait, bus) = timeleft.first().unwrap();
    let waitbus = *wait * bus;
    println!("Waiting {} minutes for bus {}. {}", wait, bus, waitbus);
    // assert_eq!(295, waitbus);

    // Experiment how slicing works.
    fn mypr(vv: &Vec<usize>) {
        println!("V {:?}", vv);
    }
    let xx = vec![2, 3, 4, 5];
    println!("{:?}", &xx);
    println!("{:?}", &xx[1..3]);
    println!("{:?}", &xx[..1]);
    println!("{:?}", &xx[..]);
    mypr(&xx);
    mypr(&xx[1..3].to_vec());

    // for timetest in 1060000..1070000 {
    // for timetest in 0..1070000 {
    //     if check_timestamp(timetest, &busses) {
    //         println!("Found!");
    //     }
    // }

    let mut timetest = 0;
    let mut nr_of_busses = 1;
    let mut stepsize: usize = 1;
    if check_timestamp(timetest, &busses[..nr_of_busses].to_vec()) {
        println!("Found!");
    }

    while nr_of_busses < busses.len() {
        // We assume the bus numbers are mutually prime.
        stepsize *= busses[nr_of_busses - 1].0;
        let timemax = stepsize * busses[nr_of_busses].0;
        nr_of_busses += 1;

        println!("Adding {:?}", busses[nr_of_busses - 1]);
        println!("Stepsize: {} {}", stepsize, timemax);

        while !check_timestamp(timetest, &busses[..nr_of_busses].to_vec()) {
            timetest += stepsize;
            if timetest > timemax {
                break;
            }
        }

        if check_timestamp(timetest, &busses[..nr_of_busses].to_vec()) {
            println!("Found! {}", timetest);
        } else {
            println!("Not found");
            break;
        }
    }


}
