from pprint import pprint

fn = "example.txt"

data1 = open(fn).read()

data2 = data1.split("\n\n")
print(len(data2))

numbers1 = data2[0]
numbers2 = [int(c) for c in numbers1.split(",")]
numbers = numbers2
print(numbers)


def parse_board(board):
    board2 = board.strip().split("\n")
    board3 = [
        [int(cell) for cell in line.split()]
        for line in board2
    ]
    return board3

boards1 = [board for board in data2[1:] if board]
boards2 = [parse_board(board) for board in boards1]
boards = boards2
pprint(boards)
