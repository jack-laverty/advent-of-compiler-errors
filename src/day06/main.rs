
fn part_1(win: usize) { // win = sliding window size
    let s = include_str!("../../input/day06.txt");
    for i in 0..s.len()-win {
        let mut x = s[i..i+win].chars().collect::<Vec<char>>();
        x.sort();
        x.dedup();
        if x.len() == win {
            println!("{}", i+win);
            break;
        }
    }
}

fn part_2() {
    part_1(14);
}

fn main() {
    part_1(4);
    part_2();
}
