use std::io;

fn main() {
    //prompt for user input
    println!("Enter temperature in fahrenheit!");
    let mut f = String::new();
    io::stdin().read_line(&mut f)
    .expect("Failed to read line");

    //convert from string to integer
    let f: u32 = f.trim().parse()
        .expect("Please enter a number!");

    //convert temperature
    let c: f32 = (f - 32) as f32 * 5.0/9.0;

    //print result
    println!("temperature {}F is converted to {}C", f, c);
}
