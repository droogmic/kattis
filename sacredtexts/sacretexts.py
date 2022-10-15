import logging

logging.basicConfig(level="INFO")

ord_range = ord('z') - ord('a') + 1
rune_offset = ord("!") - 1

def rune_ord(string):
    return sum(ord(s) - rune_offset for s in string)

def ord_letter(val):
    ord_offset = (val - ord('a')) % ord_range
    return chr(ord('a') + ord_offset)

def rune_letter(string, conversion):
    if string == "0":
        return " "
    if string == "<":
        return ","
    if string == ">":
        return "."
    val = rune_ord(string)
    val += conversion
    return ord_letter(val)

def translate(rune, letter, lines):
    conversion = ord(letter) - rune_ord(rune)
    logging.info(f"{conversion=}")
    return [
        "".join(rune_letter(r, conversion) for r in line.split())
        for line in lines
    ]

def main():
    rune, letter = input().split()
    lines = []
    while True:
        try:
            lines.append(input())
        except EOFError:
            break
    logging.info(f"{lines=}")
    for line in translate(rune, letter, lines):
        print(line)


if __name__ == "__main__":
    main()