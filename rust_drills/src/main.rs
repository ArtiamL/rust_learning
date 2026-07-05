fn main() {
    let name = String::from("Luke");

    let print_name = || println!("Name: {}", name);
    print_name();
    print_name();
    println!("Still usable: {}", name);

    let consume_name = move || println!("Consumed: {}", name);
    consume_name();
    // println!("{}", name);
}
