pub fn part1(input: &String) -> usize {
    let mut h = 0;
    let mut d = 0;

    for line in input.lines() {
        let (op, mag) = line.split_once(" ").unwrap();
        let mag = mag.parse::<usize>().unwrap();
        match op {
            "forward" => h = h + mag,
            "down" => d = d + mag,
            "up" => d = d - mag,
            _ => unreachable!(),
        }
    }

    h * d
}

pub fn part2(input: &String) -> usize {
    let mut h = 0;
    let mut d = 0;
    let mut aim = 0;

    for line in input.lines() {
        let (op, mag) = line.split_once(" ").unwrap();
        let mag = mag.parse::<usize>().unwrap();
        match op {
            "forward" => {
                h = h + mag;
                d = d + aim * mag;
            }
            "down" => aim = aim + mag,
            "up" => aim = aim - mag,
            _ => unreachable!(),
        }
    }

    h * d
}
