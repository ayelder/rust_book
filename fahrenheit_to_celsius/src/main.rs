use std::io;

fn main() {
    let celsius: f64 = loop {
        println!("Enter Fahrenheit value to convert to Celsius:");

        let mut celsius = String::new();

        io::stdin()
            .read_line(&mut celsius)
            .expect("Failed to read line");

        break match celsius.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    };
    // (celsius × 9/5) + 32 = fahrenheit
    let fahrenheit = ((celsius * 9.0) / 5.0) + 32.0;

    println!("Fahrenheit: {fahrenheit}");
}
