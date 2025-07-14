use std::io;

fn main() {
    loop {
        println!("Enter the length of the Fibonacci sequence:");

        let mut input = String::new();
        let mut t1 = 0;
        let mut t2 = 1;
        let mut next_term = t1 + t2;

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        for _ in 1..input {
            println!("{next_term}");
            t1 = t2;
            t2 = next_term;
            next_term = t1 + t2;
        }
    }
}
