from pprint import pprint
import numpy

fn = "example.txt"
#fn = "puzzle.txt"
data1 = open(fn).readlines()
nlines = len(data1)
print(nlines, nlines/2)
data2 = [
    [int(c) for c in line.strip()]
    for line in data1
]
data3 = numpy.array(data2)
data = data3
pprint(data)


sdatat = data.sum(axis=0)
print(sdatat)
sdatat2 = sdatat > nlines / 2
print(sdatat2)

sdat3 = sdatat2[::-1]
print(sdat3)
gamma = sum(c*2**i for i, c in enumerate(sdat3))
epsilon = sum((~c)*2**i for i, c in enumerate(sdat3))
power = gamma * epsilon
print(f"{gamma=}")
print(f"{epsilon=}")
print(f"{power=}")
