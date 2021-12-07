from pprint import pprint
import numpy

fn = "example.txt"
fn = "puzzle.txt"
data1 = open(fn).readlines()
# print(nlines, nlines/2)
data2 = [
    [int(c) for c in line.strip()]
    for line in data1
]
# pprint(data)


def getcommon(data, digit=0, prefer=1):
    nlines = len(data)
    data3 = numpy.array(data)
    sdatat = data3.sum(axis=0)
    # print(sdatat)
    num = sdatat[digit]
    need = nlines / 2
    print(sdatat), num, need
    if num == need:
        return prefer
    if num > need:
        return 1
    return 0

data_oxygen = data2.copy()
digit = 0
while len(data_oxygen) > 1:
    pprint(data_oxygen)
    num = getcommon(data_oxygen, digit=digit)
    print(len(data_oxygen), digit, num)
    data_oxygen = [
        line for line in data_oxygen if line[digit] == num
    ]
    digit += 1

print(data_oxygen)
oxygen = sum(c*2**i for i, c in enumerate(data_oxygen[0][::-1]))
print(f"{oxygen=}")

data_co2 = data2.copy()
digit = 0
while len(data_co2) > 1:
    pprint(data_co2)
    num = 1 - getcommon(data_co2, digit=digit)
    print(len(data_co2), digit, num)
    data_co2 = [
        line for line in data_co2 if line[digit] == num
    ]
    digit += 1

print(data_co2)
co2 = sum(c*2**i for i, c in enumerate(data_co2[0][::-1]))
print(f"{co2=}")

lifesupport = oxygen * co2
print(f"{lifesupport=}")
