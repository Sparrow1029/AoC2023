import re
from utils import get_puzzle_lines

# make sure not to miss first/last letter of the replaced digits
# example:
#
#   "abconeight5seveninexyz"
#
#   should map to:
#       "abco1e8t5s7n9exyz"
#   not:
#       "abc1ight57inexyz"
num_map = {
    "one": "o1e",
    "two": "t2o",
    "three": "t3e",
    "four": "f4r",
    "five": "f5e",
    "six": "s6x",
    "seven": "s7n",
    "eight": "e8t",
    "nine": "n9e",
}


def get_first_and_last_digit(line: str) -> int:
    if (first := re.search(r"\d", line)) is not None:
        if (second := re.search(r"\d", line[::-1])) is not None:
            return int(f"{first.group()}{second.group()}")
    return 0


def replace_digit_names(line: str) -> str:
    for k, v in num_map.items():
        line = re.sub(rf"{k}", v, line)
    return line


def part1() -> int:
    return sum(get_first_and_last_digit(line) for line in get_puzzle_lines(1))


def part2() -> int:
    return sum(
        get_first_and_last_digit(replace_digit_names(line))
        for line in get_puzzle_lines(1)
    )


if __name__ == "__main__":
    print(part1())
    print(part2())
