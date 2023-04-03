use std::fs::File;
use std::io::{ self, BufRead, BufReader };

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    let file = File::open(filename).unwrap();
    return io::BufReader::new(file).lines();
}

fn main() {
    // Read the input as vector of <usize>
    let lines = read_lines("./input.txt".to_string());
    let mut vec = Vec::new();
    for line in lines {
        vec.push(line.unwrap().parse::<usize>().unwrap());
    }
    // Sort the vector
    vec.sort();
    // Binary Search to find the two numbers that sum up two 2020
    let mut left = 0;
    let mut right = vec.len()-1;
    while left<right {
        let sum = vec[left]+vec[right];
        if sum>2020 {
            right -= 1;
        } else if sum<2020 {
            left += 1;
        } else {
            break;
        }
    }
    println!("{}\t{}", vec[left], vec[right]);
    println!("{}", vec[left]*vec[right]);
}
