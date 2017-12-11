use std::io::{ BufRead, BufReader };
use std::fs::File;
use std::ops::Add;

fn part_1<T: AsRef<[usize]>>(input: &[T]) -> usize {
    input
        .iter()
        .map(AsRef::as_ref)
        .map(|row| row.iter().max().unwrap() - row.iter().min().unwrap())
        .fold(0, Add::add)
}

fn part_2<T: AsRef<[usize]>>(input: &[T]) -> usize {
    input
        .iter()
        .map(AsRef::as_ref)
        .map(|row| row[..row.len() - 1]
             .iter()
             .enumerate()
             .filter_map(|(i, &n)| {
                 row[i + 1..]
                     .iter()
                     .filter_map(|&k| if k % n == 0 {
                         Some(k / n)
                     } else if n % k == 0 {
                         Some(n / k)
                     } else {
                         None
                     })
                     .next()
             })
             .next()
             .unwrap()
        )
        .fold(0, Add::add)
}

fn main() {
    let f = File::open("input.txt").expect("input file");
    let mut br = BufReader::new(f);
    let mut line = String::new();
    let mut rows = Vec::new();

    while let Ok(n) = br.read_line(&mut line) {
        if n == 0 { break }

        let mut cols = Vec::new();

        for col in line.split_whitespace() {
            cols.push(col.parse::<usize>().expect("a number"));
        }

        rows.push(cols);
        line.clear();
    }

    println!("part 1: {}", part_1(&rows));
    println!("part 2: {}", part_2(&rows));
}
