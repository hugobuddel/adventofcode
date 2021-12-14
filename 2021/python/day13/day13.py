"""--- Day 13: Transparent Origami ---

You reach another volcanically active part of the cave. It would be nice if you could do some kind of thermal imaging so you could tell ahead of time which caves are too hot to safely enter.

Fortunately, the submarine seems to be equipped with a thermal camera! When you activate it, you are greeted with:

Congratulations on your purchase! To activate this infrared thermal imaging
camera system, please enter the code found on page 1 of the manual.

Apparently, the Elves have never used this feature. To your surprise, you manage to find the manual; as you go to open it, page 1 falls out. It's a large sheet of transparent paper! The transparent paper is marked with random dots and includes instructions on how to fold it up (your puzzle input). For example:

6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5

The first section is a list of dots on the transparent paper. 0,0 represents the top-left coordinate. The first value, x, increases to the right. The second value, y, increases downward. So, the coordinate 3,0 is to the right of 0,0, and the coordinate 0,7 is below 0,0. The coordinates in this example form the following pattern, where # is a dot on the paper and . is an empty, unmarked position:

...#..#..#.
....#......
...........
#..........
...#....#.#
...........
...........
...........
...........
...........
.#....#.##.
....#......
......#...#
#..........
#.#........

Then, there is a list of fold instructions. Each instruction indicates a line on the transparent paper and wants you to fold the paper up (for horizontal y=... lines) or left (for vertical x=... lines). In this example, the first fold instruction is fold along y=7, which designates the line formed by all of the positions where y is 7 (marked here with -):

...#..#..#.
....#......
...........
#..........
...#....#.#
...........
...........
-----------
...........
...........
.#....#.##.
....#......
......#...#
#..........
#.#........

Because this is a horizontal line, fold the bottom half up. Some of the dots might end up overlapping after the fold is complete, but dots will never appear exactly on a fold line. The result of doing this fold looks like this:

#.##..#..#.
#...#......
......#...#
#...#......
.#.#..#.###
...........
...........

Now, only 17 dots are visible.

Notice, for example, the two dots in the bottom left corner before the transparent paper is folded; after the fold is complete, those dots appear in the top left corner (at 0,0 and 0,1). Because the paper is transparent, the dot just below them in the result (at 0,3) remains visible, as it can be seen through the transparent paper.

Also notice that some dots can end up overlapping; in this case, the dots merge together and become a single dot.

The second fold instruction is fold along x=5, which indicates this line:

#.##.|#..#.
#...#|.....
.....|#...#
#...#|.....
.#.#.|#.###
.....|.....
.....|.....

Because this is a vertical line, fold left:

#####
#...#
#...#
#...#
#####
.....
.....

The instructions made a square!

The transparent paper is pretty big, so for now, focus on just completing the first fold. After the first fold in the example above, 17 dots are visible - dots that end up overlapping after the fold is completed count as a single dot.

How many dots are visible after completing just the first fold instruction on your transparent paper?
"""

import sys
from pprint import pprint
import numpy

if len(sys.argv) > 1:
    fn = sys.argv[1]
else:
    fn = "example.txt"


def print_sheet(sheet):
    for line in sheet:
        line2 = "".join("#" if cell else "." for cell in line)
        print(line2)


data1 = open(fn).read()
points1, folds1 = data1.split("\n\n")
pprint(points1)
print(len(points1))
points2 = [tuple(int(c) for c in p.split(",")) for p in points1.split("\n")]
pprint(points2)
points = numpy.array(points2)
xmax, ymax = points.max(axis=0)
print(xmax, ymax)
sheet = numpy.zeros(shape=(ymax + 1, xmax + 1), dtype='bool')
print(sheet)
for x, y in points:
    sheet[y, x] = True

print_sheet(sheet)

print(folds1)
folds2 = folds1.split("\n")
print(folds2)
folds3 = [
    line.split(" ")[2].split("=")
    for line in folds2
    if line
]
print(folds3)
folds4 = [
    (o, int(p)) for o, p in folds3
]
folds = folds4
print(folds)

for orientation, position in folds:
    print()
    print("FOLDING!")
    print(orientation, position)
    if orientation == 'y':
        sheet1 = sheet[:position]
        sheet2 = sheet[position+1:]
        print_sheet(sheet1)
        print()
        print_sheet(sheet2)
        sheet2f = sheet2[::-1, :]
        print()
        print_sheet(sheet2f)
        print()
        # sheet = sheet1 + sheet2f
        y1 = sheet1.shape[0]
        y2 = sheet2f.shape[0]
        print(sheet1.shape, sheet2f.shape)
        print(y1, y2)
        if y1 > y2:
            print(sheet1[:y2, :].shape, sheet2f.shape)
            sheet1[:y2, :] += sheet2f
            sheet = sheet1
        else:
            print(sheet2f[-y1:, :].shape, sheet1.shape)
            sheet2f[-y1:, :] += sheet1
            sheet = sheet2f
        print_sheet(sheet)
    else:
        sheet1 = sheet[:, :position]
        sheet2 = sheet[:, position + 1:]
        print_sheet(sheet1)
        print()
        print_sheet(sheet2)
        sheet2f = sheet2[:, ::-1]
        print()
        print_sheet(sheet2f)
        # sheet = sheet1 + sheet2f
        x1 = sheet1.shape[1]
        x2 = sheet2f.shape[1]
        print(sheet1.shape, sheet2f.shape)
        print(x1, x2)
        if x1 > x2:
            print(sheet1[:, :x2].shape, sheet2f.shape)
            sheet1[:, :x2] += sheet2f
            sheet = sheet1
        else:
            print(sheet2f[:,-x1:].shape, sheet1.shape)
            sheet2f[:,-x1:] += sheet1
            sheet = sheet2f
        print_sheet(sheet)
        print()
        print_sheet(sheet)

    # break

print(sheet.sum())
