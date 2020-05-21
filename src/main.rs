fn main() {
    println!("Hello, world!");

    ownership();
    control_flow();
    loops();
    another_function(1, 2);
    variables();
}

fn ownership() {
    {
        // The string literal

        // s refers to a value that is hardcoded into the text of our program.
        // It is valid from the point at which it's declared
        // (when it comes into scope) until the end of the current scope
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

        // This kind of string can be mutated
        let mut s = String::from("hello");
        s.push_str(", world!"); // appends a literal to a String
        println!("{}", s);
    }

    {
        // Integer copy

        let x = 5;

        // This creates a copy of the value 5 beause integers are simple values with known and fixed size
        let y = x;

        // These two 5 values are pushed onto the stack
        println!("x = {}, y = {}", x, y);
    }

    {
        // Moves

        // This creates a pointer on the stack and allocates memory on the heap
        let s1 = String::from("hello");

        // This creates a copy of pointer s1 on the stack
        // and does not copy the data on the heap.
        // Here, s2 refers to the same allocated memory on the heap by s1

        let s2 = s1;
        println!("{}", s2);

        // --> Rust will never automatically create deep copies of data.

        // Because of this, any automatic copying can be assumed
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

fn control_flow() {
    {
        // if else

        let number = 3;

        if number < 5 {
            println!("condition was true");
        } else {
            println!("condition was false");
        }
    }

    {
        // Condition must be a boolean

        // if number {
        //     println!("condition was false");
        // }
        // error
    }

    {
        // Multiple conditions

        let number = 3;

        if number % 4 == 0 {
            println!("divisible by 4");
        } else if number % 3 == 0 {
            println!("divisible by 3");
        } else if number % 2 == 0 {
            println!("divisible by 2");
        } else {
            println!("other");
        }
    }

    {
        // if is an expression

        let condition = true;
        let number = if condition { 5 } else { 6 };
        println!("number = {}", number);
    }

    {
        // Values in an if expression must be of the same type

        // let number = if condition { 5 } else { "six" }; // error
    }
}

fn loops() {
    {
        // loop

        // "loop" endlessly repeats a block of a code

        // loop {
        //     println!("again");
        // }
    }

    {
        // Stopping a loop with "break"

        // We can stop a loop and return a value from it
        // nb: loop is also an expression

        let mut counter = 0;
        let result = loop {
            counter += 1;
            if counter == 10 {
                break counter + 1;
            }
        };
        println!("result = {}", result);
    }

    {
        // While loop

        // Evaluating a condition on each iteration is possible
        // with a combination of loop, if, else, and break.

        // This pattern is so common that Rust has a while loop doing just that:

        let mut number = 3;
        while number != 0 {
            println!("{}", number);
            number -= 1;
        }
        println!("LIFTOFF!!!");
    }

    {
        // Looping through a collection with a while loop

        let a = [1, 2, 3, 4, 5];
        let mut index = 0;
        while index < 5 {
            println!("the value is {}", a[index]);
            index += 1;
        }
    }

    {
        // Looping through a collection with a for loop

        // for loops adds safety by preventing us from:
        // - accessing index beyond the array's end
        // - not going far enough and missing items

        let a = [1, 2, 3, 4, 5];
        for element in a.iter() {
            println!("the value is: {}", element);
        }
    }

    {
        // Using for loop with a range

        // Range is a type provided by the standard library
        // nb: rev() reverses the range

        for number in (1..4).rev() {
            println!("{}", number);
        }
        println!("LIFTOFF!!!");
    }
}

// in function signatures, you must declare the type of each parameter
fn another_function(x: i32, y: i32) {
    five();
    println!("the value of x is {} and y is {}", x, y);
}

// we also declare the type of the returned value
fn five() -> i32 {
    5 // using a semicolon here would turn this expression in a statement
      // 5; // error
}

fn variables() {
    {
        // Declaring a variable

        let x = 5;
        println!("x = {}", x);
    }

    {
        // Reassigning with "mut"

        let mut x = 5;
        println!("the value of x is {}", x);
        x = 6;
        println!("the value of x is {}", x);
    }

    {
        // Constants

        const MAX_POINTS: u32 = 100_000;
        println!("MAX_POINTS = {}", MAX_POINTS);
    }

    {
        // Shadowing

        let x = 6;
        println!("x = {}", x);

        // we redeclare the variable x
        let x = 7;
        println!("x = {}", x);

        // x = 8; // error
        // --> we have to use "mut" to reassign x
    }

    {
        // Shadowing allows to change the type of the variable

        // Because we're effectively creating a new variable,
        // it spares us from having to come up with different names
        let spaces = "   ";
        let spaces = spaces.len();
        println!("spaces:{}", spaces);

        // If we change the type using mut:

        // let mut spaces = "   ";
        // spaces = spaces.len(); // error
        // println!("{}", spaces);

        // Rust is a statically typed language
        // --> It must know the types of all variables at compile time
    }

    {
        // In cases when many types are possible, we must add a type annotation

        let guess: u32 = "42".parse().expect("Not a number");
        println!("{}", guess);
    }

    {
        // Floating-point numbers

        let x = 2.0; // f64, by default, double precision
        println!("x:{}", x);
        let x: f32 = 3.0; // f32, single-precision float
        println!("x:{}", x);
    }

    {
        // Booleans (1 byte in size)

        let x = true;
        println!("x:{}", x);
        let x: bool = false;
        println!("x:{}", x);
    }

    {
        // Characters (4 bytes in size)

        let x = 'z';
        println!("x:{}", x);
    }

    // Rust has 2 primitive compound types
    // tuples and arrays

    {
        // Tuples

        // tuples have a fixed length
        // tuples can have different types

        let x: (i32, f64, u8) = (500, 6.4, 1);
        println!("x.0{}", x.0);
        let (a, b, c) = x;
        println!("a={} b={} c={}", a, b, c);
    }

    {
        // Arrays

        // arrays have a fixed length
        // an array is a single chunk of memory allocated on the stack (not on the heap)

        let x = [1, 2, 3, 4, 5];
        println!("x = {:?}", x);

        // an array has the same type for all its values
        // let x = [1, 2, 3, 4, 'a']; // error
    }

    {
        // Initializing an array

        let x: [i32; 5] = [1, 2, 3, 4, 5];
        println!("x = {:?}", x);
    }

    {
        // Creating an array that contains the same value for each element

        let x = [3; 5]; // same as let x = [3, 3, 3, 3, 3]
        println!("x = {:?}", x);
    }

    {
        // Accessing array elements

        let x = [1, 2, 3, 4, 5];
        let first = x[0];
        let second = x[1];
        println!("first: {} second: {}", first, second);
    }

    {
        // Invalid array element access

        let x = [1, 2, 3, 4, 5];
        let index = 10;
        println!("x = {:?}, index = {}", x, index);

        // This will panic at runtime:
        // let element = x[index]; // panic
        // println!("the value of element is:{}", element);

        // As a safety principle, rust checks that the index < array length and
        // exits immediately otherwise.

        // In many low-level languages, this check is not done and invalid
        // memory can be accessed
    }
}
