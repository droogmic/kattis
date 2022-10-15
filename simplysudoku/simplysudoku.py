import logging
import os

logging.basicConfig(level=os.environ.get("LOG_LEVEL", "WARNING"))


all_val = set(range(1, 10))


class Board:
    def __init__(self, lines):
        self.lines = lines

    def __str__(self):
        return "\n".join(
            [" ".join((f"{c}" if c else ".") for c in l) for l in self.lines]
        )

    def get(self, col_idx, row_idx):
        """from top left"""
        return self.lines[row_idx][col_idx]

    def set_val(self, col_idx, row_idx, val):
        """from top left"""
        assert self.lines[row_idx][col_idx] == 0, f"{col_idx=}, {row_idx=}, {val=}"
        self.lines[row_idx][col_idx] = val

    def get_row(self, row_idx):
        return self.lines[row_idx]

    def get_col(self, col_idx):
        return [self.get(col_idx=col_idx, row_idx=row_idx) for row_idx in range(9)]

    def get_sub_grid(self, col_idx, row_idx):
        row_lo_idx = row_idx - (row_idx % 3)
        row_range = (row_lo_idx, row_lo_idx + 3)
        col_lo_idx = col_idx - (col_idx % 3)
        col_range = (col_lo_idx, col_lo_idx + 3)
        return [
            self.get(col_idx=c, row_idx=r)
            for r in range(*row_range)
            for c in range(*col_range)
        ]

    def get_restrictions(self, col_idx, row_idx):
        restrictions = (
            set(self.get_row(row_idx))
            | set(self.get_col(col_idx))
            | set(self.get_sub_grid(col_idx, row_idx))
        )
        restrictions.remove(0)
        return restrictions

    def single_value_rule_cell(self, col_idx, row_idx) -> bool:
        if self.get(col_idx=col_idx, row_idx=row_idx):
            return False
        restrictions = self.get_restrictions(col_idx, row_idx)
        if len(restrictions) == 8:
            val = (all_val - restrictions).pop()
            self.set_val(col_idx, row_idx, val)
            logging.debug(f"single_value_rule_cell {col_idx=} {row_idx=} {val=}")
            return True
        return False

    def single_value_rule(self) -> bool:
        return any(
            self.single_value_rule_cell(col_idx, row_idx)
            for col_idx in range(9)
            for row_idx in range(9)
        )

    def unique_location_rule_row(self, row_idx, val) -> bool:
        unqiue_col_idx = None
        for col_idx in range(9):
            if self.get(col_idx=col_idx, row_idx=row_idx):
                continue
            if val in self.get_restrictions(col_idx, row_idx):
                continue
            if unqiue_col_idx is not None:
                return False
            unqiue_col_idx = col_idx
        if unqiue_col_idx is None:
            return False
        self.set_val(unqiue_col_idx, row_idx, val)
        logging.debug(f"unique_location_rule_row {unqiue_col_idx=} {row_idx=} {val=}")
        return True

    def unique_location_rule_col(self, col_idx, val) -> bool:
        unqiue_row_idx = None
        for row_idx in range(9):
            if self.get(col_idx=col_idx, row_idx=row_idx):
                continue
            if val in self.get_restrictions(col_idx, row_idx):
                continue
            if unqiue_row_idx is not None:
                return False
            unqiue_row_idx = row_idx
        if unqiue_row_idx is None:
            return False
        self.set_val(col_idx, unqiue_row_idx, val)
        logging.debug(f"unique_location_rule_col {col_idx=} {unqiue_row_idx=} {val=}")
        return True

    def unique_location_rule_sub_grid(self, sub_grid, val) -> bool:
        unqiue_location = None
        row_base, col_base = sub_grid
        row_range = (3 * row_base, 3 * row_base + 3)
        col_range = (3 * col_base, 3 * col_base + 3)
        for row_idx in range(*row_range):
            for col_idx in range(*col_range):
                if self.get(col_idx=col_idx, row_idx=row_idx):
                    continue
                if val in self.get_restrictions(col_idx, row_idx):
                    continue
                if unqiue_location is not None:
                    return False
                unqiue_location = (col_idx, row_idx)
        if unqiue_location is None:
            return False
        self.set_val(unqiue_location[0], unqiue_location[1], val)
        logging.debug(f"unique_location_rule_sub_grid {unqiue_location=} {val=}")
        return True

    def unique_location_rule(self) -> bool:
        return (
            any(
                self.unique_location_rule_row(row_idx, val)
                for row_idx in range(9)
                for val in all_val
            )
            or any(
                self.unique_location_rule_col(col_idx, val)
                for col_idx in range(9)
                for val in all_val
            )
            or any(
                self.unique_location_rule_sub_grid(sub_grid, val)
                for sub_grid in [(x, y) for x in range(3) for y in range(3)]
                for val in all_val
            )
        )

    def apply_rules(self):
        while self.single_value_rule() or self.unique_location_rule():
            logging.info(f"{self}")

    def is_complete(self):
        return all(
            self.get(col_idx, row_idx) for col_idx in range(9) for row_idx in range(9)
        )


def main():
    lines = []
    while True:
        try:
            lines.append([int(v) for v in input().split()])
        except EOFError:
            break
    assert len(lines) == 9
    assert len(lines[0]) == 9
    board = Board(lines)
    board.apply_rules()
    if board.is_complete():
        print("Easy")
    else:
        print("Not easy")
    print(board)


if __name__ == "__main__":
    main()
