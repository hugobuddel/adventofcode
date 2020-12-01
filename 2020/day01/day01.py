"""Day 1 of Advent of Code 2020."""
from typing import List


def find_2020(report: List[int]):
    return report


def test_example():
    report = [
        1721,
        979,
        366,
        299,
        675,
        1456,
    ]
    fix = find_2020(report)
    assert fix == 514579


if __name__ == '__main__':
    test_example()
