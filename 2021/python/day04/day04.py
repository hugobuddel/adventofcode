from pprint import pprint
import sys

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


def check_number(board, number):
    board = [
        [
            -1 if cell == number else cell
            for cell in line
        ]
        for line in board
    ]
    return board


def check_board_lines(board):
    for line in board:
        if sum(line) == -5:
            return True
    return False


def check_board(board):
    board_trans = list(zip(*board))
    if check_board_lines(board):
        return True
    if check_board_lines(board_trans):
        return True
    return False


for number in numbers:
    print("Drawing", number)
    pprint(boards)
    boards_old = boards
    boards = []
    for board in boards_old:
        board = check_number(board, number)
        boards.append(board)
        if check_board(board):
            board2 = [
                0 if cell == -1 else cell
                for line in board
                for cell in line
            ]
            sum_board = sum(board2)
            prod = sum_board * number
            print(number, sum_board, prod)
            sys.exit()
