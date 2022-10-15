class Board:
    def __init__(self, lines):
        self.lines = lines

    @property
    def rows(self):
        for idx in range(9):
            yield self.lines[idx]


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
    print(list(board.rows))


if __name__ == "__main__":
    main()
