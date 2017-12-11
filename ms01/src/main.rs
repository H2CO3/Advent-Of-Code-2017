use std::io::Read;
use std::fs::File;
use std::ops::Add;

fn part_1(digits: &[u8]) -> usize {
    (0..digits.len())
        .map(|i| if digits[i] == digits[(i + 1) % digits.len()] {
            (digits[i] - b'0') as usize
        } else {
            0
        })
        .fold(0, Add::add)
}

fn part_2(digits: &[u8]) -> usize {
    (0..digits.len())
        .map(|i| if digits[i] == digits[(i + digits.len() / 2) % digits.len()] {
            (digits[i] - b'0') as usize
        } else {
            0
        })
        .fold(0, Add::add)
}

fn main() {
    let mut f = File::open("input.txt").expect("input file");
    let mut digits = Vec::new();
    f.read_to_end(&mut digits).expect("read file");

    if digits.last() == Some(&b'\n') { digits.pop(); }

    println!("Part 1: {}", part_1(&digits));
    println!("Part 2: {}", part_2(&digits));
}
