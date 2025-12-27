Celsius to Fahrenheit CLI
A simple Rust practice project focused on learning how to handle user input and string parsing without the program crashing.

What I was practicing:
parse() and Result: Understanding that .parse() returns an enum, not just a value.

Error Handling: Using a match statement to catch bad inputs (like letters) instead of using .expect() to let it panic.

Diverging Arms: Using std::process::exit(1) so the compiler allows the match to exit early if the parsing fails.

The Code
The core of the logic is this block:

Rust

let userinp: f64 = match userinp.trim().parse() {
    Ok(num) => num,
    Err(_) => {
        println!("Invalid input. Please enter a number.");
        process::exit(1);
    }
};
