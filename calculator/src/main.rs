use std::io;
use regex::Regex; // Not available as part of standard.

fn main() {
    println!("Welcome to my calculator!");

    loop {
        println!("Enter an expression to calculate eg - 2 + 2 or 2 * 2 and so on: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim();

        if input == "exit" {
            println!("Goodbye!!");
            break;
        }

        let result = calculate(input);
        println!("Result: {}", result);
    }
}

fn calculate(input: &str) -> f64 {
    // Use regex to parse the input
    // Extract numbers
    // Do the calculation and return values.
    let re = Regex::new(r"([\d.]+)\s*([+-/*])\s*([\d.]+)").unwrap();
    let captures = re.captures(input).unwrap();

    let num1 :f64 = captures[1].parse().unwrap();
    let num2 :f64 = captures[3].parse().unwrap();
    let op :&str = &captures[2];

    // Match and return
    match op {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => num1 / num2,
        _   => panic!("Invalid Operator"),
    }
}