import sys

file = "input.txt"
#file = "example.txt"

with open(file) as f:
    # 415,863 -> 942,863

    grid = [[0] * 1000 for _ in range(1000)]

    for line in f.readlines():
        s = line.split()
        p1 = s[0].split(',')
        x1, y1 = int(p1[0]), int(p1[1])

        p2 = s[2].split(',')
        x2, y2 = int(p2[0]), int(p2[1])

        if x1 == x2:
            if y1 > y2:
                itr = range(y2, y1 + 1)
            else:
                itr = range(y1, y2 + 1)

            for i in itr:
                grid[x1][i] += 1
        elif y1 == y2:
            if x1 > x2:
                itr = range(x2, x1 + 1)
            else:
                itr = range(x1, x2 + 1)

            for i in itr:
                grid[i][y1] += 1
        else:
            pass

    count = 0
    for row in grid:
        for y in row:
            if y >= 2:
                count += 1
    
    print(count)