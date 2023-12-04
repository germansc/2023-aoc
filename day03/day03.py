NON_SYMBOLS = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.']
PARTS = []

def symbol_in_string(s):
    for i in range(0, len(s)):
        if s[i] not in NON_SYMBOLS:
            return True
    return False


def validate_part(schem, x, y, l):
    valid = False
    part_num = int(schem[y][x:x+l])

    # Check West, East, North and South
    if x != 0 and schem[y][x-1] not in NON_SYMBOLS:
        valid = True
    elif x+l < len(schem[y]) and schem[y][x+l] not in NON_SYMBOLS:
        valid = True
    elif y != 0 and symbol_in_string(schem[y-1][max(0,x-1):min(len(schem[y]),x+l+1)]):
        valid = True
    elif (y != len(schem)-1) and symbol_in_string(schem[y+1][max(0,x-1):min(len(schem[y]),x+l+1)]):
        valid = True
    return (valid, part_num)

def analyze_schem(schem):
    x = 0
    y = 0
    while y < len(schem):
        line = schem[y]
        while x < len(line):
            if schem[y][x].isdigit():
                e = x + 1;
                while e < len(line) and schem[y][e].isdigit():
                    e += 1
                # Found a number and length.
                # print(f"Found a number of len {e-x} at {y},{x}")
                PARTS.append(validate_part(schem, x, y, e-x))
                x = e+1
            else:
                x += 1
        y += 1
        x = 0


# PART 2:
GEARS = []

def get_part(s,x,y):
    start = x
    end = x
    while(start > -1 and s[y][start].isdigit()):
        start-=1

    # Find the end:
    while(end < len(s[y]) and s[y][end].isdigit()):
        end+=1

    # Parse
    return int(s[y][start+1:end])


def validate_gear(s,x,y):
    parts = []

    # Check in the eigth possible points
    if x != 0 and s[y][x-1].isdigit():
        parts.append(get_part(s, x-1, y))
    if x != len(s[y])-1 and s[y][x+1].isdigit():
        parts.append(get_part(s, x+1, y))

    if x != 0 and y != 0 and s[y-1][x-1].isdigit():
        parts.append(get_part(s, x-1, y-1))
    if y != 0 and s[y-1][x].isdigit():
        parts.append(get_part(s, x, y-1))
    if y != 0 and x != len(s[y])-1 and s[y-1][x+1].isdigit():
        parts.append(get_part(s, x+1, y-1))

    if x != 0 and y != len(s)-1 and s[y+1][x-1].isdigit():
        parts.append(get_part(s, x-1, y+1))
    if y != len(s)-1 and s[y+1][x].isdigit():
        parts.append(get_part(s, x, y+1))
    if y != len(s)-1 and x != len(s[y])-1 and s[y+1][x+1].isdigit():
        parts.append(get_part(s, x+1, y+1))

    # Assuming that part numbers are unique...
    parts = set(parts)
    return (len(parts) == 2, parts)


def find_gear(schem):
    for y in range(0,len(schem)):
        for x in range(0,len(schem[y])):
            if schem[y][x] == '*':
               GEARS.append(validate_gear(schem, x, y))

# Main entry point....
if __name__ == "__main__":
    print("Day 03! - The shame!")
    part1 = 0
    part2 = 0

    with open('input.txt') as f:
        schem = [line.rstrip() for line in f]

    analyze_schem(schem)
    for part in PARTS:
        # print(part)
        if part[0] is True:
            part1 += part[1]

    find_gear(schem)
    for gear in GEARS:
        # print(gear)
        if gear[0] is True:
            lis = list(gear[1])
            part2 += lis[0]*lis[1]

    print(f"----------------")
    print(f"Part 1: {part1}")
    print(f"Part 2: {part2}")

