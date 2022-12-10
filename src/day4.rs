fn parse(input: &str) -> impl Iterator<Item = ((usize, usize), (usize, usize))> + '_ {
    input.lines().map(|s| {
        let (a, b) = s.split_once(",").unwrap();
        let (al, ar) = a.split_once("-").unwrap();
        let (bl, br) = b.split_once("-").unwrap();
        (
            (al.parse::<usize>().unwrap(), ar.parse::<usize>().unwrap()),
            (bl.parse::<usize>().unwrap(), br.parse::<usize>().unwrap()),
        )
    })
}

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    parse(input)
        .map(|((al, ar), (bl, br))| {
            if (al <= bl && ar >= br) || (al >= bl && ar <= br) {
                1
            } else {
                0
            }
        })
        .sum()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    parse(input)
        .map(|((al, ar), (bl, br))| {
            if al <= br && ar >= bl || al >= br && ar <= bl {
                1
            } else {
                0
            }
        })
        .sum()
}
