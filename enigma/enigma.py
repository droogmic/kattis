import logging
import os
from typing import Dict, Tuple

logging.basicConfig(level=os.environ.get("LOG_LEVEL", "INFO"))


class Crossword(Dict[Tuple[int, int], str]):
    def __init__(row, cols):
        self.rows = rows
        self.cols = cols


def solve(crossword, words):
    for row_idx in range(rows):
        word_len = None
        for col_idx in range(cols):
            if crossword[(row_idx, col_idx)] == ".":
                word_len = word_len or 0
                word_len += 1
            elif crossword[(row_idx, col_idx)] == "#":
                if word_len is not None:
                    possible_words = [w for w in words if len(w) == word_len]
                word_len = None
            else:
                raise NotImplementedError()


def main():
    rows, cols = (int(v) for v in input().split())
    crossword = Crossword(rows, cols)
    for row_idx in range(rows):
        row = input()
        for col_idx in range(cols):
            crossword[(row_idx, col_idx)] = row[col_idx]
    num_words = int(input())
    words = [input() for _ in range(num_words)]
    crossword = solve(crossword, words)


if __name__ == "__main__":
    main()
