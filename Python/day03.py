from utils import get_puzzle_lines

SYMBOLS = ["*", "$", "+", "&", "-", "=", "%", "#", "@", "/"]


def parse_input():
    lines = list(get_puzzle_lines(3))
    for y, line in enumerate(lines):
        for x, val in enumerate(line):
            print(f"({x}, {y}): {val}")


if __name__ == "__main__":
    parse_input()
