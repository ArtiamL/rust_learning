fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}
