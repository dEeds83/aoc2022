use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("/Users/fabiandietz/Documents/rust/github.com/dEeds83/aoc2022/day01/input.txt")
        .expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");
    let split = data.split('\n');
    let mut current:i32 = 0;
    let mut top1:i32 = 0;
    let mut top2:i32 = 0;
    let mut top3:i32 = 0;
    for line in split{
        if line == "" {
            if current > top1 {
                top3 = top2;
                top2 = top1;
                top1 = current;
            } else if current > top2 {
                top3 = top2;
                top2 = current;
            } else if current > top3 {
                top3 = current;
            }
            current = 0;
            continue;
        }
        let val = line.parse::<i32>().unwrap();
        current += val;
    }
    println!("Part A MaxCalories: {}", top1);
    println!("Part B Top3 Calories Sum: {}", top1+top2+top3);
}
