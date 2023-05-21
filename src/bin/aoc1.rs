use std::fs::read_to_string;

fn main() {
    aoc1();
}

fn aoc1() {
    let calories_input = read_to_string("input1.txt").expect("Failed to read input file");
    // parse each line into a number unless its a blank line
    let calories: Vec<i32> = calories_input
        .lines()
        .map(|line| line.parse::<i32>().unwrap_or_else(|_| 0))
        .collect();
    println!("Calories: {:?}", calories);
    // split the calories at every 0
    let mut calorie_splits: Vec<Vec<i32>> = Vec::new();
    let mut current_split: Vec<i32> = Vec::new();
    for calorie in calories {
        if calorie == 0 {
            calorie_splits.push(current_split);
            current_split = Vec::new();
        } else {
            current_split.push(calorie);
        }
    }
    calorie_splits.push(current_split);
    println!("Calorie splits: {:?}", calorie_splits);
    // calculate the total calories for each split
    let mut total_calories: Vec<i32> = Vec::new();
    for split in calorie_splits {
        let mut total = 0;
        for calorie in split {
            total += calorie;
        }
        total_calories.push(total);
    }
    println!("Total calories: {:?}", total_calories);

    // sort the total calories
    total_calories.sort();
    println!("Sorted total calories: {:?}", total_calories);
    // add up last 3 total calories
    let mut total = 0;
    for i in 1..4 {
        total += total_calories[total_calories.len() - i];
    }
    println!("Total calories for last 3 splits: {:?}", total);
}