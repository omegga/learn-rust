pub fn integer_copy() {
    let x = 5;
    let y = x;

    // --> We don't have to use .clone() here
    // Since integers have a known size at compile time,
    // they are stored entirely on the stack,
    // and copies of the actual values are quick to make
    println!("x = {}, y = {}", x, y); // x is still valid and wasn't moved into y

    // because x is an integer, it has the Copy trait
    // if a type has the Copy trait,
    // an older variable is still usable after assignment
}

pub fn deep_copy() {
    // If we need a deep copy of the heap data of the String,
    // we can use the .clone() method
    let s1 = String::from("hello");
    let s2 = s1.clone(); // that code may be expensive
    println!("s1 = {}, s2 = {}", s1, s2);
}
