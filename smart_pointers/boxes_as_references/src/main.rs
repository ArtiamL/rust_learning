use std::ops::Deref;

struct MyBox<T>(T);
struct CustomSmartPointer {
    data: String,
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}!", self.data);
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    //Same as above
    let a = 5;
    let b = Box::new(x);

    assert_eq!(5, a);
    assert_eq!(5, *b);

    //Using custom Box
    let c = 5;
    let d = MyBox::new(x);

    assert_eq!(5, c);
    assert_eq!(5, *d);

    //Deref coercion, converts &String to &str
    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    //Showcase what happens when drop is called
    let csp = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let dsp = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointers created");
}
