pub fn ownership_vs_copy() {
    // s comes into scope
    let s = String::from("hello");

    // value of s moves into the function
    takes_ownership(s);
    // s is no longer valid here
    // (a compile-time error would happen if we use s here)
    // println!("s = {}", s); // error

    // x comes into scope
    let x = 5;

    // x would move into the function
    makes_copy(x);
    // but since i32 is Copy, it's okay to still use x afterward
    println!("x = {}", x);
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
    // some_string goes out of scope and 'drop' is called.
    // The backing memory is freed
}

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
    // some_integer goes out of scope. Nothing special happens
}

pub fn returning_values() {
    // gives_ownership moves its return value into s1
    let s1 = gives_ownership();
    println!("{}", s1);

    let s2 = String::from("hello"); // s2 comes into scope

    // s2 is moved into takes_and_gives_back, which also
    // moves its return value into s3
    let s3 = takes_and_gives_back(s2);
    println!("s3 = {}", s3);
    // println!("s2 = {}", s2); // error
}

fn gives_ownership() -> String {
    // gives_ownership will move its return value into the function
    // that calls it
    let some_string = String::from("hello"); // some_string comes into scope

    // some_string is returned and moves out to the calling function
    some_string
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope
    a_string // a_string is returned and moves out to the calling function
}
