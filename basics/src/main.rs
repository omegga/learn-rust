fn main() {
    println!("Hello, world!");
    declaring_a_variable();
    reassigning_variables();
    shadowing();
    shadowing_2();
    constants();
    type_annotations();
    floating_point_numbers();
    booleans();
    chars();
    tuples();
    arrays();
    array_initialization();
    accessing_array_elements();
    invalid_array_element_access();
    if_else();
    if_else_if();
    if_expression();
    some_function(1, 2);
}

fn declaring_a_variable() {
    // Declaring a variable

    let x = 5;
    println!("x = {}", x);
}

fn reassigning_variables() {
    // We can reassign variables using the "mut" keyword

    let mut x = 5;
    println!("the value of x is {}", x);
    x = 6;
    println!("the value of x is {}", x);
}

fn shadowing() {
    // Shadowing

    let x = 6;
    println!("x = {}", x);

    // we redeclare the variable x
    let x = 7;
    println!("x = {}", x);

    // x = 8; // error
    // --> we still have to use "mut" to reassign x
}

fn shadowing_2() {
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

fn constants() {
    // Constants

    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS = {}", MAX_POINTS);
}

fn type_annotations() {
    // In cases when many types are possible, we must add a type annotation

    let guess: u32 = "42".parse().expect("Not a number");
    println!("{}", guess);
}

fn floating_point_numbers() {
    // Floating-point numbers

    let x = 2.0; // f64, by default, double precision
    println!("x:{}", x);
    let x: f32 = 3.0; // f32, single-precision float
    println!("x:{}", x);
}

fn booleans() {
    // Booleans (1 byte in size)

    let x = true;
    println!("x:{}", x);
    let x: bool = false;
    println!("x:{}", x);
}

fn chars() {
    // Characters (4 bytes in size)

    let x = 'z';
    println!("x:{}", x);
}

fn tuples() {
    // Tuples

    // tuples have a fixed length
    // and can have different types

    let x: (i32, f64, u8) = (500, 6.4, 1);
    println!("x.0{}", x.0);
    let (a, b, c) = x;
    println!("a={} b={} c={}", a, b, c);
}

fn arrays() {
    // Arrays

    // arrays have a fixed length
    // an array is a single chunk of memory allocated on the stack (not on the heap)

    let x = [1, 2, 3, 4, 5];
    println!("x = {:?}", x);

    // an array has the same type for all its values
    // let x = [1, 2, 3, 4, 'a']; // error
}

fn array_initialization() {
    // Initializing an array

    let x: [i32; 5] = [1, 2, 3, 4, 5];
    println!("x = {:?}", x);

    // Creating an array that contains the same value for each element

    let x = [3; 5]; // same as let x = [3, 3, 3, 3, 3]
    println!("x = {:?}", x);
}

fn accessing_array_elements() {
    // Accessing array elements

    let x = [1, 2, 3, 4, 5];
    let first = x[0];
    let second = x[1];
    println!("first: {} second: {}", first, second);
}

fn invalid_array_element_access() {
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

fn if_else() {
    // if else
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Condition must be a boolean
    // if number {
    //     println!("condition was false");
    // }
    // error
}

fn if_else_if() {
    // if else if
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

fn if_expression() {
    // if is an expression
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number = {}", number);

    // Values in an if expression must be of the same type
    // let number = if condition { 5 } else { "six" }; // error
}

// in function signatures, you must declare the type of each parameter
fn some_function(x: i32, y: i32) {
    another_function();
    println!("the value of x is {} and y is {}", x, y);
}

// we also declare the type of the returned value
fn another_function() -> i32 {
    5 // using a semicolon here would turn this expression in a statement
      // 5; // error
}
