use std::io;

// This program converts temperature between degrees
// Fahrenheit and Celsius

fn main() {
    // ask the user which they are using, create a variable input, and read it
    println!("input \"fahrenheit\" or \"celsius\":");

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("failed to read");

    //ask for a number, create variable for number, read it
    println!("provide the number value:");

    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("failed to read");

    //convert input to a number

    let number: i32 = number.trim().parse().expect("please enter a number");

    //equations for conversion

    if choice.trim() == "fahrenheit" {
        let celsius_ans = ({number} - 32) * 5 / 9;
        println!("your input as celsius is {celsius_ans}");
    } else {
        let fahrenheit_ans = ({number} * 9 / 5) + 32;
        println!("your input as fahrenheit is {fahrenheit_ans}");
    }
}
