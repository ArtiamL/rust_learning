fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // Causes code to panic as index is out of bounds
    // let does_not_exist = &v[100];

    // Code doen't panic as get() returns None
    let does_not_exist = v.get(100);

    // This won't compile as an immutable reference to the vector is held. Vectors are contiguous
    // blocks in memory, adding a new elem may require allocating new memory and copying vals from
    // old location, violating the borrow checker rules.
    // let first = &v[0];
    // v.push(6);
    // println!("The first element is: {first}");

    // Loop over vector with immutable references
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    // Iterate v as mutable to increment each elem by 50
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
}
