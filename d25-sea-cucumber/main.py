grid = [list(inp.strip()) for inp in open("input.txt").readlines()]

def step(grid):
    height = len(grid)
    width = len(grid[0])

    moved = False

    next = [list('.' * width) for _ in range(height)]
    for y in range(height):
        for x in range(len(grid[y])):
            # going left to right will make this work!

            if grid[y][x] == '>':
                if grid[y % height][(x + 1) % width] == '.':
                    next[y][(x+1) % width] = '>'
                    moved = True
                else:
                    next[y][x] = '>'

    for y in range(height):
        for x in range(width):
            if grid[y][x] == 'v':
                if next[(y+1) % height][x] == '.' and grid[(y+1) % height][x] != 'v':
                    moved = True
                    next[(y+1) % height][x] = 'v'
                else:
                    next[y][x] = 'v'

    return moved, next

def show(grid):
    for row in grid:
        print("".join(row))
    print("\n")

for i in range(1000):
    moved, grid = step(grid)

    if not moved:
        print(i + 1)
        show(grid)
        break
