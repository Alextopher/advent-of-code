lines = [ line[:-1] for line in open("example.txt").readlines() ]

for line in lines:
    s1, s2, s3, s4 = [], [], [], []

    for c in line:
        if c == '(':
            s1.append(0)