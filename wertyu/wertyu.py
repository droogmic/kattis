ROWS = [
    "`1234567890-=",
    "QWERTYUIOP[]\\",
    "ASDFGHJKL;'",
    "ZXCVBNM,./",
]


def convert(char: str) -> str:
    if char == " ":
        return " "
    for ROW in ROWS:
        if char in ROW:
            return ROW[ROW.find(char) - 1]
    raise NotImplementedError(f"Unknown character '{char}'")


def main():
    line = input()
    line = "".join(convert(char) for char in line)
    print(line)


if __name__ == "__main__":
    main()
