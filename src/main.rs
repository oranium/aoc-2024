use std::{collections::HashMap, fs};

fn main() {
    day_1();
}

fn day_1() {
    // parse the file into the two lists
    // contents looks something like
    // 123  456
    // 789  012
    let contents = fs::read_to_string("./res/input_day1_1").unwrap();
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in contents.lines() {
        match line
            .split_whitespace()
            .map(|slice| slice.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
            .as_slice()
        {
            &[left_mem, right_mem] => {
                left.push(left_mem);
                right.push(right_mem);
            }
            _ => {
                panic!("no")
            }
        }
    }
    list_distance(left.clone(), right.clone());
    list_similarity(left.clone(), right.clone());
}
fn list_distance(mut left: Vec<i32>, mut right: Vec<i32>) {
    left.sort();
    right.sort();
    let res = left
        .iter()
        .zip(right.iter())
        .map(|(x, y)| (x - y).abs())
        .reduce(|a, b| a + b)
        .unwrap();
    println!(
        "Day 1, Part 1 - The distance between the two lists is: {:?}",
        res
    );

    // senior historians mark
    // locations on their
    // list with a star.
    // the chief is in one
    // of the first 50 places
    // locations are listed by ID
    //
    //left_list.sort();
    //right_list.sort();
    //let it = left_list.iter().zip(right_list.iter());
    //it.reduce(|x, y| -> u16 { (x - y).abs() })
}
/*
 * This time, you'll need to figure out exactly how often each number from the
 * left list appears in the right list. Calculate a total similarity score by
 * adding up each number in the left list after multiplying it by the number of
 * times that number appears in the right
 * list.
 */
fn list_similarity(left: Vec<i32>, right: Vec<i32>) {
    let mut frequencies = HashMap::new();
    right.iter().for_each(|val| {
        let count = frequencies.get(val).unwrap_or(&0).to_owned() + 1;
        frequencies.insert(val, count);
    });
    let score = left
        .iter()
        .map(|num| {
            let factor = frequencies.get(num).unwrap_or(&0);
            factor * num
        })
        .reduce(|a, b| a + b)
        .unwrap();
    println!("Day 1, Part 2 - The similarity score is: {:?}", score);
}
