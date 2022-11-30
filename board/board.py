import logging
from dataclasses import dataclass
from typing import Optional
from enum import Enum

logging.basicConfig(level="INFO")


class Clr(Enum):
    RED = "R"
    GREEN = "G"
    BLUE = "B"


@dataclass
class Cell:
    final: Optional[Clr] = None
    erased: bool = False

    def __repr__(self):
        if self.erased:
            return "+"
        if self.final is None:
            return "W"
        return self.final.value

    def is_solved(self):
        return self.final is None or self.erased

    @classmethod
    def from_str(cls, s: str):
        if s == "W":
            return cls(final=None)
        return cls(final=Clr(s))


class Canvas(dict):
    def __init__(self, rows, cols):
        self.rows = rows
        self.cols = cols
        super().__init__()

    def __repr__(self):
        return "\n" + "\n".join(
            "".join(
                str(self[(row_idx, col_idx)]) 
                for col_idx in range(self.cols)
            )
            for row_idx in range(self.rows)
        )

    def get_3x3(self, row_mid, col_mid):
        retval = []
        for row_idx in range(row_mid-1, row_mid+2):
            for col_idx in range(col_mid -1, col_mid+2):
                retval.append(self[(row_idx, col_idx)])
        return retval

    def is_solved(self):
        return all(cell.is_solved() for cell in self.values())


def solve(rows, cols, canvas) -> bool:
    while not canvas.is_solved():
        logging.info(f"{canvas=}")
        for row_idx in range(1, rows-1):
            for col_idx in range(1, cols-1):
                cells = canvas.get_3x3(row_idx, col_idx)
                target = None
                for cell in cells:
                    if not cell.is_solved():
                        target = cell.final
                        break
                else:
                    continue
                logging.debug(f"{target=}")
                if target is None:
                    continue
                if any(cell.final != target and not cell.erased for cell in cells):
                    continue
                for cell in cells:
                    cell.erased = True
                break
            else:
                continue
            break
        else:
            return False
    logging.info(f"final {canvas=}")
    return True


def main() -> None:
    rows, cols = (int(v) for v in input().split())
    canvas = Canvas(rows, cols)
    for row_idx in range(rows):
        line = input()
        for col_idx in range(cols):
            canvas[(row_idx, col_idx)] = Cell.from_str(line[col_idx])
    logging.info(f"initial {canvas=}")
    if solve(rows, cols, canvas):
        print("YES")
    else:
        print("NO")


if __name__ == "__main__":
    main()
