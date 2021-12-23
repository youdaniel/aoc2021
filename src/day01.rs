pub fn part1(input: &String) -> usize {
    let report: Vec<usize> = input.lines().map(|l| l.parse::<usize>().unwrap()).collect();
    report.windows(2).filter(|w| w[1] > w[0]).count()
}

pub fn part2(input: &String) -> usize {
    let report: Vec<usize> = input.lines().map(|l| l.parse::<usize>().unwrap()).collect();
    report.windows(4).filter(|w| w[3] > w[0]).count()
}
