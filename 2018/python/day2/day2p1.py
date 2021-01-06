
def countletters(s, n):
    """
    Is there a letter that appears exactly n times?
    """
    for l in s:
        if s.count(l) == n:
            return True

    return False

def getchecksum(theids):
    counts3 = 0
    counts2 = 0

    for myid in theids:
        if countletters(myid, 3):
            counts3 += 1
        if countletters(myid, 2):
            counts2 += 1

    print(counts3, counts2, counts3 * counts2)


ss = """
abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab
"""

myids = ss.split()
getchecksum(myids)

myids2 = open('input.txt').readlines()
getchecksum(myids2)
