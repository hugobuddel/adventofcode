import itertools

def compareids(id1, id2):
    """
    Compare how many characters are the different between two ids.
    """
    return sum(c1 != c2 for c1, c2 in zip(id1, id2))

def getcommon(id1, id2):
    """
    Return common characters
    """
    return "".join(c1 for c1, c2 in zip(id1, id2) if c1 == c2)


def findcorrectid(theids):
    for myid1, myid2 in itertools.combinations(theids, 2):
        cc = compareids(myid1, myid2)
        samec = getcommon(myid1, myid2)
        print(myid1, myid2, cc, samec)
        if cc == 1:
            break


ss = """
abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz
"""

myids = ss.strip().split()

findcorrectid(myids)

myids2 = open('input.txt').readlines()
findcorrectid(myids2)
