with open("input.txt", "r") as file:
    data = file.read()

position = 50
counter_p1 = 0
counter_pt2 = 0
position_overlap = 0

instructions = data.splitlines() 
for line in instructions:
    direction, operation = line[0], int(line[1:])
    if direction == 'L':
        position -= operation
    elif direction == 'R':
        position += operation
    if position < 0 or position >= 100:
        position_overlap = abs(position // 100)
        position %= 100
        counter_pt2 += position_overlap
    if position == 0:
        counter_p1 += 1
    # print(position, position_overlap, counter_pt2)
print(f"Answer for part 1 is: {counter_p1}")
print(f"Answer for part 2 is: {counter_pt2}")



position = 50
counter_p1 = 0
counter_pt2 = 0
position_overlap = 0

instructions = data.splitlines() 
for line in instructions:
    direction, operation = line[0], int(line[1:])
    if direction == 'L':
        position -= operation
    elif direction == 'R':
        position += operation

    while position < 0:
        position += 100
        counter_pt2 += 1
    if position == 0:
        counter_pt2 += 1
        
    while position >= 100:
        position -= 100
        counter_pt2 += 1

    if position == 0:
        counter_p1 += 1
    # print(position, position_overlap, counter_pt2)
print(f"Answer for part 1 is: {counter_p1}")
print(f"Answer for part 2 is: {counter_pt2}")
