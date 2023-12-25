"""
--- Part Two ---

Your calculation isn't quite right. It looks like some of the digits are actually spelled
out with letters: one, two, three, four, five, six, seven, eight, and nine also count as
valid "digits".

Equipped with this new information, you now need to find the real first and last digit on
each line. For example:

two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen

In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding these
together produces 281.

What is the sum of all of the calibration values?
"""

NUMBERS = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("zero", 0),
    ("0", 0),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
]

# basically a shitty b tree
NUM_MAP = {}  # like {"t": [("wo", 2),] ...}
for word, num in NUMBERS:
    if word[0] not in NUM_MAP:
        NUM_MAP[word[0]] = []
    NUM_MAP[word[0]].append((word[1:], num))


def line_val(line: str) -> int:
    first_num = None
    for idx in range(len(line)):
        if line[idx] in NUM_MAP:
            for rem, num in NUM_MAP[line[idx]]:
                if not rem or line[idx+1:idx+1+len(rem)] == rem:
                    first_num = num
                    break
        if first_num:
            break

    last_num = None
    for idx in range(len(line)-1, -1, -1):
        if line[idx] in NUM_MAP:
            for rem, num in NUM_MAP[line[idx]]:
                if not rem or line[idx+1:idx+1+len(rem)] == rem:
                    last_num = num
                    break
        if last_num:
            break

    if first_num is None or last_num is None:
        raise ValueError("Couldn't find a number in line: {}".format(line), first_num, last_num)
    return int(str(first_num) + str(last_num))


def sum_lines(lines: str):
    return sum(line_val(l) for l in lines.splitlines() if l)


def test_example():
    ex_input = """two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"""
    assert sum_lines(ex_input) == 281


def main():
    with open("01/b_input.txt") as f:
        print(sum_lines(f.read()))


if __name__ == "__main__":
    main()
