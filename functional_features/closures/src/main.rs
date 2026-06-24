use std::{thread, time::Duration};

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for colour in &self.shirts {
            match colour {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    //Closure examples

    let _expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    println!("\n\nClosure examples:");
    let example_closure = |x| x;

    let _s = example_closure(String::from("hello"));
    // Can't change type as type is inferred to be String from previous line.
    // let n = example_closure(5);

    //Immutably borrows
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");

    //Mutably borrows
    let mut list2 = vec![1, 2, 3];
    println!("\nBefore defining closure: {list2:?}");
    let mut borrows_mutably = || list2.push(7);

    borrows_mutably();
    println!("After calling closure: {list2:?}");

    //Move keyword
    let list3 = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("\nFrom thread: {list:?}"))
        .join()
        .unwrap();
}
