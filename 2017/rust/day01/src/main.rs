//! --- Day 1: Inverse Captcha ---
//
// The night before Christmas, one of Santa's Elves calls you in a panic. "The
// printer's broken! We can't print the Naughty or Nice List!" By the time you
// make it to sub-basement 17, there are only a few minutes until midnight. "We
// have a big problem," she says; "there must be almost fifty bugs in this
// system, but nothing else can print The List. Stand in this square, quick!
// There's no time to explain; if you can convince them to pay you in stars,
// you'll be able to--" She pulls a lever and the world goes blurry.
//
// When your eyes can focus again, everything seems a lot more pixelated than
// before. She must have sent you inside the computer! You check the system
// clock: 25 milliseconds until midnight. With that much time, you should be
// able to collect all fifty stars by December 25th.
//
// Collect stars by solving puzzles. Two puzzles will be made available on each
// day millisecond in the Advent calendar; the second puzzle is unlocked when
// you complete the first. Each puzzle grants one star. Good luck!
//
// You're standing in a room with "digitization quarantine" written in LEDs
// along one wall. The only door is locked, but it includes a small interface.
// "Restricted Area - Strictly No Digitized Users Allowed."
//
// It goes on to explain that you may only leave by solving a captcha to prove
// you're not a human. Apparently, you only get one millisecond to solve the
// captcha: too fast for a normal human, but it feels like hours to you.
//
// The captcha requires you to review a sequence of digits (your puzzle input)
// and find the sum of all digits that match the next digit in the list. The
// list is circular, so the digit after the last digit is the first digit in
// the list.
//
// For example:
//
//     1122 produces a sum of 3 (1 + 2) because the first digit (1) matches the
//     second digit and the third digit (2) matches the fourth digit.
//
//     1111 produces 4 because each digit (all 1) matches the next.
//
//     1234 produces 0 because no digit matches the next.
//
//     91212129 produces 9 because the only digit that matches the next one is
//     the last digit, 9.
//
// What is the solution to your captcha?

// --- Part Two ---
//
// You notice a progress bar that jumps to 50% completion. Apparently, the door
// isn't yet satisfied, but it did emit a star as encouragement. The
// instructions change:
//
// Now, instead of considering the next digit, it wants you to consider the
// digit halfway around the circular list. That is, if your list contains 10
// items, only include a digit in your sum if the digit 10/2 = 5 steps forward
// matches it. Fortunately, your list has an even number of elements.
//
// For example:
//
//     1212 produces 6: the list contains 4 items, and all four digits match the digit 2 items ahead.
//     1221 produces 0, because every comparison is between a 1 and a 2.
//     123425 produces 4, because both 2s match each other, but no other digit has a match.
//     123123 produces 12.
//     12131415 produces 4.
//
// What is the solution to your new captcha?

fn captcha(digits: &str) -> u32 {
    let dign: Vec<u32> = digits.chars().map(|x| x.to_digit(10).unwrap()).collect();

    let mut dign2: Vec<u32> = dign.clone();
    let start = dign2.remove(0);
    dign2.push(start);

    dign.iter().zip(dign2.iter()).filter(|x| x.0 == x.1).map(|x| x.0).sum()
}

fn captcha2(digits: &str) -> u32 {
    let dign: Vec<u32> = digits.chars().map(|x| x.to_digit(10).unwrap()).collect();
    let mut dign1 = dign.clone();
    let mut dign2 = dign1.split_off(dign1.len() / 2);
    dign2.append(&mut dign1);

    dign.iter().zip(dign2.iter()).filter(|x| x.0 == x.1).map(|x| x.0).sum()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_captcha1() {
        assert_eq!(3, captcha("1122"));
    }

    #[test]
    fn test_captcha2() {
        assert_eq!(4, captcha("1111"));
    }

    #[test]
    fn test_captcha3() {
        assert_eq!(0, captcha("1234"));
    }

    #[test]
    fn test_captcha4() {
        assert_eq!(9, captcha("91212129"));
    }

    #[test]
    fn test_captcha5() {
        assert_eq!(6, captcha2("1212"));
    }

    #[test]
    fn test_captcha6() {
        assert_eq!(0, captcha2("1221"));
    }

    #[test]
    fn test_captcha7() {
        assert_eq!(4, captcha2("123425"));
    }

    #[test]
    fn test_captcha8() {
        assert_eq!(12, captcha2("123123"));
    }

    #[test]
    fn test_captcha9() {
        assert_eq!(4, captcha2("12131415"));
    }

}

fn main() {
    println!("Advent of Code 2017 Day 1!");

    let sequence = "5228833336355848549915459366737982598312959583817455621545976784792489468198365998232722734876612332352376192813552949814275947575774339529811976644361517795586998319242241614813622734255797569571577699238592667287428166398221572885869416419682687759743978434571821267146514338394624525648338739929479912368172669885577319718389278168766844487948761697438722556857882433224393723131298876252626643517236883999115665656935521675772866516185899317132494716723615493476397115627687887665194781746377341468995954554518252916859227397693885254329628812355612487594445522395853551734567498838382248616137969637971369615443599973588326388792893969924855316437952313492551671545714262784738343517166544197194547173515155927244175447296474282154114951181648317875827525814453758846194548872789943372281952995222779173812444186491115426476188672253249744478946863317915136832199132868917891243591195719354721129116229164688256853628339233919671468781913167415624214152793864585332944468428849171876873433621524242289488135675313544498245498637424139153782925723745249728743885493877792648576673196889949568317234125863369187953788611841388353999875519172896329524346527265231767868839696693328273381772726782949166112932954356923757485139367298699922984925977724972944277991686823219295939734313874834861796179591659174726432357533113896212781566659154939419866797488347448551719481632572231632463575591599696388223344219228325134233238538854289437756331848887242423387542214691157226725179683638967415678697625138177633444765126223885478348951332634398291612134852858683942466178329922655822225426534359191696177633167962839847985826676955417426617126288255366123169174674348417932158291334646767637764323226842771523598562429399935789788215958367362467652444854123951972118358417629679454978687337137675495295768451719631999398617828287671937584998697959425845883145736323818225129311845997214987663433375689621746665629187252511643969315283316269222835744532431378945137649959158495714472963839397214332815241141327714672141875129895";
    let solution = captcha(sequence);
    println!("Captcha 1: {}", solution);
    let solution2 = captcha2(sequence);
    println!("Captcha 2: {}", solution2);
}
