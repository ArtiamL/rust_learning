fn main() {
    // std::string::String is implemented as a wrapper around std::vec::Vec
    // Strings are UTF-8 encoded

    // Create new empty String
    let _empty_s = String::new();

    // to_string() is available on types that implement the `Display` trait, such as string
    // literals.
    let data = "initial contents";
    let _s = data.to_string();
    // The following also works
    let _s = "initial contents".to_string();

    // String::from() also allows creating a String from a string literal
    let _s = String::from("initial contents");

    // Strings can grow in size:
    let mut s = String::from("foo");
    s.push_str("bar"); // s is now "foobar"

    // push_str takes string slice as ownership is not necessarily needed to be taken. This allows
    // s2 to be used after to print
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    // push method takes single char and adds to String
    let mut s = String::from("lo");
    s.push('l');

    // + can also be used to concat Strings
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let _s3 = s1 + &s2;

    // For concatenating multiple Strings, format! is preferred
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let _s = format!("{s1}-{s2}-{s3}");

    // Due to UTF-8 encoding, Rust does not allow traditional String indexing; rather, use ranges
    // to get particular bytes of a String
    // The following String requires 2 bytes per char
    let hello = "Здравствуйте";
    let start = &hello[0..4]; // Returns Зд

    // Best way to operate on pieces of Strings is to be explicit over wanting chars or bytes
    // For individual Unicode scalar vals use chars()
    for c in start.chars() {
        println!("{c"); // prints 3,д (comma indicates new line here)
    }

    for b in start.bytes() {
        println!("{b}"); // prints 208,151,208,180 (comma indicates new line here)
    }
}
