use std::io;
use std::collections::HashMap;

fn main() {
    //ask for vector size
    println!("Enter list size!");
    let mut size = String::new();
    let size = read_input(&mut size);

    //ask for list data
    let mut list = Vec::new();
    let mut count = 0;
    println!("Enter list of integers!");

    while count < size {
        let mut input = String::new();
        let input = read_input(&mut input);

        //store in the vector
        list.push(input);
        count += 1;
    }

    //compute median
    let med = median(&mut list);

    let mut map = HashMap::new();

    //convert vector list to hashmap
    for int in &list {
        let count = map.entry(int).or_insert(0);
        *count += 1;
    }

    //get key with most occurrence
    let mode = map.iter()
        .max_by_key(|entry | entry.1)
        .unwrap();

    println!("mode is {}, median is {}", mode.0, med);
}

//function to read user input
fn read_input (input: &mut String) -> i32 {
    //get user input
    io::stdin().read_line(input)
        .expect("failed to read line!");

    //convert string to integer
    let input: i32 = input.trim()
        .parse()
        .expect("Please enter a number!");

    input //return input
}

//function to return median value in an unsorted list
fn median (list: &mut Vec<i32>) -> i32 {
    list.sort(); //sort the vector

    //obtain list size
    let size = list.len();

    //compute median index
    let med_index = {
        if size % 2 == 0{
            size / 2
        }
        else{
            (size + 1) / 2
        }
    };

    list[med_index - 1] //return median
}
