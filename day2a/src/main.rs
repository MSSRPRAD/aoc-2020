use std::fs::File;
use std::io::{ self, BufRead, BufReader };
#[macro_use] extern crate scan_fmt;

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    let file = File::open(filename).unwrap();
    return io::BufReader::new(file).lines();
}

fn main() {
    // Read the input as vector of <usize>
    let lines = read_lines("./input.txt".to_string());
    let mut vec = Vec::new();
    for line in lines {
        vec.push(line.unwrap());
    }
    let mut count: usize = 0;
    for i in 0..vec.len() {
        let (a, b, c, d) = scan_fmt_some!(&vec[i], "{d}-{d} {}: {}", usize, usize, String, String);
        let mut freq = 0;
        for j in 0..d.clone().unwrap().len(){
           if d.clone().unwrap().chars().nth(j).unwrap() == c.clone().unwrap().chars().nth(0).unwrap() {
            freq +=1;
           }
        }
        if freq >= a.unwrap() as usize && freq <= b.unwrap() as usize {
            count +=1;
        }
    }
    println!("{:?}", count);

}
