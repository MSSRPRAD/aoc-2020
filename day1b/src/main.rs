use std::fs::File;
use std::io::{self, BufRead, BufReader};

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
    let mut first: usize = 0;
    let mut left: usize;
    let mut right: usize;
    while first < vec.len() - 1 {
        let mut vec1 = vec.clone();
        vec1.remove(first);
        left = 0;
        right = vec1.len() - 1;
        while left < right {
            let sum = vec1[left] + vec1[right] + vec[first];
            if sum < 2020 {
                left += 1;
            } else if sum > 2020 {
                right -= 1;
            } else {
                break;
            }
        }
        if vec1[left] + vec1[right] + vec[first] == 2020 {
            println!(
                "{}\t{}\t{}\t{}",
                vec1[left],
                vec1[right],
                vec[first],
                vec1[left] + vec1[right] + vec[first]
            );
            println!("{}", vec1[left] * vec1[right] * vec[first]);
            break;
        } else {
            first += 1;
        }
    }
}
