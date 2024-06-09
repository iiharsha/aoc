input = list()

with open('02.txt', 'r') as f:
    for line in f:
        input.append(line.rstrip())
#12 red cubes, 13 green cubes, and 14 blue cubes
def part1():
    CUBES = {
        "red": 12,
        "green": 13,
        "blue": 14,
    }
    
    game_ids = list()

    for line in input:
        valid = True
        id = int(line.split(':')[0].split(' ')[1])

        games = line.split(':')[1].split(';')
        for game in games:
            cubes = game.split(',')
            for cube in cubes:
                split = cube.strip().split(' ')
                colour = split[1]
                total = int(split[0])
                
                if CUBES[colour] < total:
                    valid = False
                    break
            
            if not valid:
                break

        if valid:
            game_ids.append(id)

    return sum(game_ids)



def part2():
    powers = list()

    for line in input:
        colors = {
            "red": 0,
            "green": 0,
            "blue": 0,
        }
        games = line.split(":")[1].split(';')
        for game in games:
            cubes = game.split(',')
            for cube in cubes:
                split = cube.strip().split(' ')

                color = split[1]
                total = int(split[0])

                if colors[color] < total:
                    colors[color] = total

        powers.append(colors["red"] * colors["green"] * colors["blue"])

    return sum(powers)

print(f"Part1: {part1()}")
print(f"Part2: {part2()}")
