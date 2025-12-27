use std::io;
use std::process;

fn main() {
    let mut userinp = String::new();

    println!("Convert Celsius to Farenheit");

    io::stdin()
        .read_line(&mut userinp)
        .expect("Failed to read line");

    let userinp: f64 = match userinp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("That is not a valid number. Try again");
            process::exit(1);
        }
    };

    convert(userinp);
}


fn convert(userinp: f64) {
    let transformed: f64 = (userinp * 1.8 ) + 32.0;
    println!("Your temperature in Farenheit is: {transformed}");

}
