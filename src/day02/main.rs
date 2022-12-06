use std::collections::HashMap;

pub fn part_1() {
    let lns = include_str!("../../input/day02.txt").lines();
    let mut map = HashMap::new();
    map.insert("A X", 3 + 1);
    map.insert("A Y", 6 + 2);
    map.insert("A Z", 0 + 3);
    map.insert("B X", 0 + 1);
    map.insert("B Y", 3 + 2);
    map.insert("B Z", 6 + 3);
    map.insert("C X", 6 + 1);
    map.insert("C Y", 0 + 2);
    map.insert("C Z", 3 + 3);
    let mut points = 0;
    for i in lns {
        points += map.get(i).unwrap();
    }
    println!("{}", points);
}

pub fn part_2() {
    let lns = include_str!("../../input/day02.txt").lines();
    let mut map = HashMap::new();
    map.insert("A X", 3 + 0);
    map.insert("A Y", 1 + 3);
    map.insert("A Z", 2 + 6);
    map.insert("B X", 1 + 0);
    map.insert("B Y", 2 + 3);
    map.insert("B Z", 3 + 6);
    map.insert("C X", 2 + 0);
    map.insert("C Y", 3 + 3);
    map.insert("C Z", 1 + 6);
    let mut points = 0;
    for i in lns {
        points += map.get(i).unwrap();
    }
    println!("{}", points);
}

fn main() {
    part_1();
    part_2();
}
