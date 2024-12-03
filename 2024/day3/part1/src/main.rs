use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let lines: Vec<String> = read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    let pattern1 = Regex::new(r"mul\((\d+,\d+)\)").unwrap();
    let pattern2 = Regex::new(r"mul\((\d+)\,(\d+)\)").unwrap();

    let mut finalvec1 = mul_parse(lines, pattern1);
    let finalint = mul_process(finalvec1, pattern2);
    println!("{:?}", finalint)
}

fn mul_parse(thelines: Vec<String>, pattern: Regex) -> Vec<String> {
    let mut mulvec: Vec<String> = Vec::new();
    for i in thelines {
        let tempvec: Vec<String> = pattern
            .find_iter(&i)
            .filter_map(|fr| fr.as_str().parse::<String>().ok())
            .collect();

        for i2 in tempvec {
            mulvec.push(i2);
        }
    }
    mulvec
}

fn mul_process(mulvec: Vec<String>, pattern: Regex) -> i64 {
    let mut mulint: i64 = 0;
    for i in mulvec {
        let captures = pattern.captures(&i).unwrap();
        let capture1 = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();

        let capture2 = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
        mulint += (capture1 * capture2) as i64;
    }

    mulint
}
