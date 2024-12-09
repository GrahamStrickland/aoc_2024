use std::fs;
use std::io::{self, prelude::*, BufReader};

pub fn read_file(file_name: &str) -> String {
    let content = fs::read_to_string(file_name).expect("Error reading file {file_name}");
    content
}

pub fn read_lists(content: &[u8]) -> io::Result<(Vec<i32>, Vec<i32>)> {
    return Ok(BufReader::new(content)
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut pair = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
            (pair.next().unwrap(), pair.next().unwrap())
        })
        .collect::<(Vec<i32>, Vec<i32>)>());
}

pub fn list_diff(list1: &mut Vec<i32>, list2: &mut Vec<i32>) -> i32 {
    let mut diff = 0;

    list1.sort();
    list2.sort();

    for (item1, item2) in list1.iter().zip(list2.iter()) {
        diff += (item2 - item1).abs()
    }

    diff
}

pub fn day1() {
    let content = read_file("data/day1.tsv");
    let read_res = read_lists(content.as_bytes());
    match read_res {
        Ok(mut res) => {
            let list_res = list_diff(&mut res.0, &mut res.1);
            println!("Day 1: result = {list_res}");
        }
        Err(e) => panic!("{e}"),
    }
}
