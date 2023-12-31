"""Day 1: Report Repair.

After saving Christmas five years in a row, you've decided to take a vacation at
a nice resort on a tropical island. Surely, Christmas will go on without you.

The tropical island has its own currency and is entirely cash-only. The gold
coins used there have a little picture of a starfish; the locals just call them
stars. None of the currency exchanges seem to have heard of them, but somehow,
you'll need to find fifty of these coins by the time you arrive so you can pay
the deposit on your room.

To save your vacation, you need to get all fifty stars by December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each day
in the Advent calendar; the second puzzle is unlocked when you complete the
first. Each puzzle grants one star. Good luck!

Before you leave, the Elves in accounting just need you to fix your expense
report (your puzzle input); apparently, something isn't quite adding up.

Specifically, they need you to find the two entries that sum to 2020 and then
multiply those two numbers together.

For example, suppose your expense report contained the following:

1721
979
366
299
675
1456
In this list, the two entries that sum to 2020 are 1721 and 299. Multiplying
them together produces 1721 * 299 = 514579, so the correct answer is 514579.

Of course, your expense report is much larger. Find the two entries that sum to
2020; what do you get if you multiply them together?

--- Part Two ---
The Elves in accounting are thankful for your help; one of them even offers you
a starfish coin they had left over from a past vacation. They offer you a second
one if you can find three numbers in your expense report that meet the same
criteria.

Using the above example again, the three entries that sum to 2020 are 979, 366,
and 675. Multiplying them together produces the answer, 241861950.

In your expense report, what is the product of the three entries that sum to
2020?
"""

from typing import List, Optional


def find_sum(report: List[int], sum_wanted) -> Optional[int]:
    """Product of two numbers that sum to sum_wanted."""
    report_sorted = iter(sorted(report))
    report_reversed = iter(reversed(sorted(report)))
    front = next(report_sorted)
    back = next(report_reversed)
    while (the_sum := front + back) != sum_wanted and front < back:
        if the_sum < sum_wanted:
            front = next(report_sorted)
        else:
            back = next(report_reversed)

    if the_sum == sum_wanted:
        solution = front * back
        print("Found:", front, back, the_sum, solution)
    else:
        solution = None

    return solution


def find_2020(report: List[int]) -> int:
    """Return product of two entries that sum to 2020."""
    return find_sum(report, 2020)


def find_2020_three(report: List[int]) -> int:
    """Return product of two entries that sum to 2020."""

    front, *report = report
    while not (solution_two := find_sum(report, 2020 - front)):
        front, *report = report

    return front * solution_two


def test_example():
    """Test with example."""
    report = [
        1721,
        979,
        366,
        299,
        675,
        1456,
        # Added because otherwise solution is immediately found:
        1,
        2,
        3,
        4,
        5,
        1800,
        1801,
        1802,
    ]
    fix = find_2020(report)
    assert fix == 514579
    fix3 = find_2020_three(report)
    assert fix3 == 241861950


def day01():
    """Solve day 1."""
    report = [
        int(c)
        for c in open('input.txt').readlines()
    ]
    print(find_2020(report))
    print(find_2020_three(report))


if __name__ == '__main__':
    test_example()
    day01()
