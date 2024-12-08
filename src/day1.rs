use std::io::{self, prelude::*, BufReader};

pub fn read(content: &[u8]) -> io::Result<(Vec<i32>, Vec<i32>)> {
    return Ok(BufReader::new(content).lines().map(|line| {
        let line = line.unwrap();
        let mut pair = line.split("\t").map(|s|s.parse::<i32>().unwrap());
        (pair.next().unwrap(), pair.next().unwrap())
    }).collect::<(Vec<i32>, Vec<i32>)>())
}

pub fn list_diff(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    let mut diff = 0;
    let mut new_list1 = list1.clone();
    let mut new_list2 = list2.clone();

    new_list1.sort();
    new_list2.sort();

    for (item1, item2) in new_list1.iter().zip(new_list2.iter()) {
        diff += (item2 - item1).abs()
    }

    diff
}
