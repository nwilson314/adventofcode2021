use std::env;
use std::fs;
use std::collections::VecDeque;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    part1(&contents);
    part2(&contents);
    
}

fn part1(contents: &String) {

    let mut count: i32 = 0;
    let mut prev_line: Option<&str> = None;
    for line in contents.lines() {
        if let Some(value) = prev_line {
            if line.parse::<i32>().unwrap() > value.parse::<i32>().unwrap() {
                count += 1;
            }   
        }
        prev_line = Some(line);
    }

    println!("The number of increases is {}", count);
}

fn part2(contents: &String) {
    let mut count = 0;

    let mut window: VecDeque<i32> = VecDeque::new();

    let mut prev_window: Option<i32> = None;
    for line in contents.lines() {
        if window.len() < 3 {
            window.push_back(line.parse::<i32>().unwrap());
            continue;
        }
        let sum: i32 = window.iter().sum();
        if let Some(value) = prev_window {
            if sum > value {
                count += 1;
            }
        }
        prev_window = Some(sum);
        window.pop_front();
        window.push_back(line.parse::<i32>().unwrap());
    }

    let sum: i32 = window.iter().sum();
    if let Some(value) = prev_window {
        if sum > value {
            count += 1;
        }
    } 
    println!("The number of increases is {}", count);
}

