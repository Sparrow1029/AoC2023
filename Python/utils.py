from pathlib import Path
from typing import Iterator


def get_puzzle_input_fpath(day: int) -> Path:
    puz_input_dir = Path(__file__).parent.parent.absolute() / "puzzle_inputs"
    return puz_input_dir / f"day{day:0>2}.txt"


def get_puzzle_lines(day: int) -> Iterator[str]:
    with open(get_puzzle_input_fpath(1)) as data:
        for line in data:
            yield line.strip()
