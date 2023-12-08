def check_board(board):
    for row in board:
        if row[0][1] and row[1][1] and row[2][1] and row[3][1] and row[4][1]:
            return True

    for i in range(5):
        if board[0][i][1] and board[1][i][1] and board[2][i][1] and board[3][i][1] and board[4][i][1]:
            return True
    
    return False

with open("input.txt") as f:
    inputs = [ int(i) for i in f.readline().split(",") ]
    f.readline()
    
    boards = []
    board = []
    for line in f.readlines():
        if line == "\n":
            boards.append(board)
            board = []
        else:
            board.append([[int(i), False] for i in line.split()])
    boards.append(board)


    done = [ False ] * len(boards)

    for input in inputs:
        for i, board in enumerate(boards):
            for row in board:
                for b in row:
                    if b[0] == input:
                        b[1] = True

            if check_board(board):
                done[i] = True

            if all(done):
                s1 = 0

                for row in board:
                    for b in row:
                        if not b[1]:
                            s1 += b[0]
                
                print(s1, input, s1 * input)
                exit(1)
