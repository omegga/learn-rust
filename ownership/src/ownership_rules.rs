pub fn rules() {
    // Ownership rules in Rust:
    // - each value in Rust has a variable that's called its owner
    // - there can only be one owner at a time
    // - when the owner goes out scope, the value will be dropped
    // Assigning a value to another variable moves it
    let s1 = String::from("hello");
    let s2 = s1;
    // The line above:
    // - creates a copy of pointer s1 on the stack
    // - allocates memory on the heap
    // - does not copy the data on the heap.
    // Here, s2 refers to the same allocated memory on the heap by s1
    // --> Rust will never automatically create deep copies of data.
    // Because of this, any automatic copying can be assumed
    // to be inexpensive in terms of runtime performance

    // Here, Rust considers s1 to no longer be valid
    // This prevents s1 from freeing the same memory as s2
    // (also known as "double free error")
    // when s1 and s2 go out of scope, only s2 will free the memoy

    // println!("{}, world", s1); // error
    println!("{}, world", s2); // ok
}
