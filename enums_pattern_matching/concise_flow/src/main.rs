fn main() {
    // Single case match, lots of boilerplate
    let o_config_max = Some(3u8);
    match o_config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    // If let - syntactic sugar for more concise for single-use match expr
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
}
