use std::env;
use std::fs;

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
    let mut depth: i32 = 0;
    let mut pos: i32 = 0;

    for line in contents.lines() {
        let split_line: Vec<&str> = line.split(" ").collect();
        match split_line[0] {
            "forward" => {
                pos += split_line[1].parse::<i32>().unwrap()
            }
            "down" => {
                depth += split_line[1].parse::<i32>().unwrap()
            }
            "up" => {
                depth -= split_line[1].parse::<i32>().unwrap()
            }
            _ => {}
        }
    }
    println!("pos: {}", &pos);
    println!("depth: {}", &depth);
    let mult = pos * depth;
    println!("mult: {}", &mult);
}

fn part2(contents: &String) {
    let mut depth: i32 = 0;
    let mut pos: i32 = 0;
    let mut aim: i32 = 0;

    for line in contents.lines() {
        let split_line: Vec<&str> = line.split(" ").collect();
        match split_line[0] {
            "forward" => {
                pos += split_line[1].parse::<i32>().unwrap();
                depth += split_line[1].parse::<i32>().unwrap() * aim
            }
            "down" => {
                aim += split_line[1].parse::<i32>().unwrap()
            }
            "up" => {
                aim -= split_line[1].parse::<i32>().unwrap()
            }
            _ => {}
        }
    }
    println!("pos: {}", &pos);
    println!("depth: {}", &depth);
    let mult = pos * depth;
    println!("mult: {}", &mult);
}