pub fn the_string_type() {
    // Since string literals are immutable, we cannot use them for text
    // whose size in unknown at compile time and might change while
    // running the program.

    // Instead, we use a 2nd String type that allocates on the heap
    // String::from can create a String from a string literal

    let s = String::from("hello");
    println!("{}", s);
}

pub fn string_mutation() {
    // Strings can be mutated
    let mut s = String::from("hello");
    s.push_str(", world!"); // appends a literal to a String
    println!("{}", s);
}
