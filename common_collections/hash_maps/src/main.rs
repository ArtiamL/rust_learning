use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let _score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, val) in &scores {
        println!("{key}: {val}");
    }

    // Types that implement the Copy trait will have their values copied into a HashMap. Owned
    // values (such as String) will have vals moved and ownership overtaken by the HashMap.
    let field_name = String::from("Favourite colour");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // HashMap.entry() allows for checking if key exists before inserting val
    let mut scores_new: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");
}
