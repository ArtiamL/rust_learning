use std::io;

fn main() {
    loop {
        println!("Enter temp in farenheit:");

        let mut temp = String::new();

        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

        let temp: f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let celcius = (temp - 32.0) / 1.8;

        println!("You entered: {temp}f, which is {celcius:.2}c");
    }
}
