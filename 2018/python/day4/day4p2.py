import numpy

class Entry:
    def __init__(self, s):
        t1, s3 = s.strip().strip("[").split("]")
        self.text = s3.strip()
        d2, t2 = t1.split()
        self.year, self.month, self.day = [int(x) for x in d2.split("-")]
        self.hour, self.minute = [int(x) for x in t2.split(":")]
        print(self.year,self.month, self.day, self.hour, self.minute, self.text)

ss = """
[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up
"""

sentries1 = ss.strip().split("\n")
sentries2 = open("input.txt").readlines()
sentries = sentries1
sentries = sentries2

entries = [Entry(mys) for mys in sentries]
entries.sort(key=lambda x: (x.year, x.month, x.day, x.hour, x.minute))

guards = {}
for entry in entries:
    if entry.text.startswith("Guard"):
        guardid = entry.text.split()[1]
        print("Now using guard", guardid)
        if guardid not in guards:
            guards[guardid] = numpy.zeros(shape=(60,))
    if "falls" in entry.text:
        start = entry
        minutes_start = start.hour * 60 + start.minute
    if "wakes" in entry.text:
        end = entry
        minutes_end = end.hour * 60 + end.minute
        #guards[guardid] += minutes_end - minutes_start
        guards[guardid][minutes_start:minutes_end] += 1

print(guards)

bestid, bestminutes = sorted(guards.items(), key=lambda x: max(x[1]))[-1]
bestmin = bestminutes.argmax()
bestidnum = int(bestid.strip("#"))
print(bestid, bestmin, bestidnum * bestmin)


