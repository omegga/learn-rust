fn main() {
    println!("Hello, world!");
    ownership();
}

fn ownership() {
    {
        // The string literal

        // s refers to a value that is hardcoded
        // into the text of our program
        // it is valid from the point at which it's declared
        // (when it comes into scope)
        // until the end of the current scope
        let s = "hello";
        println!("{}", s);
    }

    {
        // The String Type

        // Since string literals are immutable, we cannot use them for text
        // whose size in unknown at compile time and might change while
        // running the program.
        // Instead, we use a 2nd String type that allocates on the heap
        // String::from can create a String from a string literal
        let s = String::from("hello");
        println!("{}", s);
    }

    {
        // String mutation

        // this kind of string can be mutated
        let mut s = String::from("hello");
        s.push_str(", world!"); // appends a literal to a String
        println!("{}", s);
    }

    {
        // Integer copy

        let x = 5;

        // this creates a copy of the value 5 beause integers are simple values with known and fixed size
        let y = x;

        // these two 5 values are pushed onto the stack
        println!("x = {}, y = {}", x, y);
    }

    {
        // Moves

        // this creates a pointer on the stack and allocates memory on the heap
        let s1 = String::from("hello");

        // this creates a copy of pointer s1 on the stack
        // and does not copy the data on the heap
        // (here, s2 refers to the same allocated memory on the heap by s1)
        let s2 = s1;
        println!("{}", s2);
        // --> Rust will never automatically create deep copies of data
        // because of this, any automatic copying can be assumed
        // to be inexpensive in terms of runtime performance
    }

    {
        // Moves (2)

        // Ownership rules in Rust:
        // - each value in Rust has a variable that's called its owner
        // - there can only be one owner at a time
        // - when the owner goes out scope, the value will be dropped

        // Assigning a value to another variable moves it
        let s1 = String::from("hello");
        let s2 = s1;

        // Here, Rust considers s1 to no longer be valid
        // This prevents s1 from freeing the same memory as s2
        // (also known as "double free error")
        // when s1 and s2 go out of scope, only s2 will free the memoy

        // println!("{}, world", s1); // error
        println!("{}, world", s2); // ok
    }

    {
        // Deep copy

        // If we need a deep copy of the heap data of the String,
        // we can use the .clone() method
        let s1 = String::from("hello");
        let s2 = s1.clone(); // that code may be expensive
        println!("s1 = {}, s2 = {}", s1, s2);
    }

    {
        // The Copy Trait

        let x = 5;
        let y = x;

        // --> We don't have to use .clone() here
        // Since integers have a known size at compile time,
        // they are stored entirely on the stack,
        // so copies of the actual values are quick to make
        println!("x = {}, y = {}", x, y); // x is still valid and wasn't moved into y

        // because x is an integer, it has the Copy trait
        // if a type has the Copy trait,
        // an older variable is still usable after assignment
    }

    {
        // Ownership vs copy

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

    {
        // Returning values

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
