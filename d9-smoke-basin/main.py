input = [line[:-1] for line in open("input.txt").readlines()]

count = 0
for y in range(len(input)):
    for x in range(len(input)):
        if y == 0:
            top = True
        else:
            top = input[y][x] < input[y-1][x]

        if y >= len(input) - 1:
            bottom = True
        else:
            bottom = input[y][x] < input[y+1][x]

        if x == 0:
            left = True
        else:
            left = input[y][x] < input[y][x-1]

        if x >= len(input) - 1:
            right = True
        else:
            right = input[y][x] < input[y][x+1]

        if top and bottom and left and right:
            count += int(input[y][x]) + 1

print(count)