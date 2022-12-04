
pub fn solve() {
    let lns = include_str!("../input/day01.txt").lines();
    let mut elf = 0;
    let mut v = Vec::new();
    for i in lns {
        if i == "" {
            v.push(elf);
            elf = 0;
        }
        else {
            elf += i.parse::<i32>().unwrap();
        }
    }
    v.sort();
    println!("part 1: {}", v.last().unwrap());

    v.reverse();
    v.truncate(3);
    println!("part 2: {}", v.iter().sum::<i32>());
}