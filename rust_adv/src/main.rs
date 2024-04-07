use std::io;

fn main() {
    println!("My Rust Calculator");


    let mut num_str =  String::new();

    io::stdin()
        .read_line(&mut num_str)
        .expect("Failed to read line");


    // Parse the string as a float
    let num: f32 = num_str.trim().parse().expect("Please enter a valid number");
    println!("You guessed: {num}");


    // operator input
    let mut operator = String::new();

    // Loop until valid operator is entered
    loop {
        println!("Enter an operator (+, -, *, /):");
        io::stdin().read_line(&mut operator).expect("Failed to read line");
        operator = operator.trim().to_lowercase(); // Make operator case-insensitive

        if ["+", "-", "*", "/"].contains(&&operator[..]) {
            break; // Exit loop on valid operator
        } else {
            println!("Invalid operator. Please enter +, -, *, or /");
            operator.clear(); // Clear invalid input for next attempt
        }
    }

    // second number
    let mut num2_str = String::new();

    // Read second number
    io::stdin().read_line(&mut num2_str).expect("Failed to read line");
    let num2: f32 = num2_str.trim().parse().expect("Please enter a valid number");

    // Perform calculation based on operator
    let result = match operator.as_str() {
        "+" => num + num2,
        "-" => num - num2,
        "*" => num * num2,
        "/" => {
            if num2 == 0.0 {
                panic!("Division by zero is not allowed!");
            } else {
                num / num2
            }
        },
        _ => unreachable!(), // Shouldn't happen due to loop check
    };

    println!("Result: {}", result);
}