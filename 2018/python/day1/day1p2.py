import itertools


def findfreq(changes):
    cchanges = itertools.cycle(changes)

    freq = 0
    freqs = {freq}
    for change in cchanges:
        freq += change
        if freq in freqs:
            print(freq)
            break

        freqs.add(freq)

# mychanges = [+3, +3, +4, -2, -4]
mychanges = [int(x) for x in open('input.txt').readlines()]
findfreq(mychanges)
