from utils import get_puzzle_lines
import re


def parse_line(line: str) -> tuple[int, int]:
    card, nums = line.split(" | ")
    card_id = re.match(r"Card\s+(\d+): ", card).group(1)  # type: ignore
    winning_nums = set(int(w) for w in re.findall(r"(\d+)", card.split(": ")[-1]))
    nums = set(int(n) for n in re.findall(r"(\d+)", nums))

    return (int(card_id), len(nums.intersection(winning_nums)))


def solve_part1():
    return sum(int(2 ** (parse_line(line)[1] - 1)) for line in get_puzzle_lines(4))


def solve_part2():
    scores = {
        card_id: nums
        for card_id, nums in (parse_line(line) for line in get_puzzle_lines(4))
    }
    final = [1] * len(scores)
    for i in range(0, len(scores)):
        for j in range(i + 1, i + scores[i + 1] + 1):
            final[j] += final[i]
    return sum(final)


if __name__ == "__main__":
    print("Part 1: ", solve_part1())
    print("Part 2: ", solve_part2())
