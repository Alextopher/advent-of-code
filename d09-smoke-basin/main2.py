input = [line[:-1] for line in open("input.txt").readlines()]

test = []
for y in range(len(input)):
    for x in range(len(input[y]) - 1):
        if y == 0:
            print("north")
            north = True
        else:
            north = input[y][x] < input[y-1][x]

        if y == len(input) - 1:
            south = True
            print("south")
        else:
            south = input[y][x] < input[y+1][x]

        if x == 0:
            print("left")
            left = True
        else:
            left = input[y][x] < input[y][x-1]

        if x == len(input[y]) - 2:
            print("right")
            right = True
        else:
            right = input[y][x] < input[y][x+1]

        print(x, y, north, south, left, right)

        if north and south and left and right:
            test.append((x, y))

def flood(visted, x, y):
    print(x,y, len(input[y]) - 1, len(input) - 1)
    if (x,y) in visted:
        return 0
    
    visted.append((x,y))

    c = 1
    if x > 0:
        if int(input[y][x-1]) < 9:
            c += flood(visted, x - 1, y)

    if x != len(input[y]) - 1:
        if int(input[y][x+1]) < 9:
            c += flood(visted, x + 1, y)

    if y > 0:
        if int(input[y-1][x]) < 9:
            c += flood(visted, x, y - 1)

    if y != len(input) - 1:
        if int(input[y+1][x]) < 9:
            c += flood(visted, x, y + 1)

    return c

s = sorted([flood([], x, y) for (x,y) in test], reverse=True)
print(s[0] * s[1] * s[2])