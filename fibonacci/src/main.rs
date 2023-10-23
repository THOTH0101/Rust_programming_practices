use std::io;

fn main() {
    //prompt for user input
    println!("Enter an index for the fibonacci number");
    let mut index = String::new();

    //read user input
    io::stdin().read_line(&mut index)
        .expect("Failed to read line");

    //parse input from string to integer
    let index: u32 = index.trim().parse()
        .expect("Please type a number!");

    //call fibonacci_number function
    let fib = fibonacci_number(index);

    //print result to console
    println!("fibonacci number for index {} is {}", index, fib);
}

fn fibonacci_number (index: u32) -> i32{
    let mut count = 0;
    let mut number = 0;
    let mut prev: i32;
    let mut next = 1;

    //using while loop to generate fibonacci number
    while count < index {
        prev = next;
        next = number;
        number = next + prev;
        count += 1;
    }
    number//return result
}
