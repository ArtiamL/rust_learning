fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");
}

// Won't compile as borrow checker can't infer lifetimes of either x or y
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}
