
fn part_1() {
    let lns = include_str!("../../input/day01.txt").lines();
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
    println!("{}", v.last().unwrap());
}

fn part_2() {
    let lns = include_str!("../../input/day01.txt").lines();
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
    v.reverse();
    v.truncate(3);
    println!("{}", v.iter().sum::<i32>());
}

fn main() {
    part_1();
    part_2();
}