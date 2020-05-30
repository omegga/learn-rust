fn main() {
    conditional_expressions();
    while_let();
    for_loops_pattern();
    let_statements();
    function_parameters_pattern(7);
    refutability();
    matching_literals();
    matching_named_variables();
    multiple_patterns();
    matching_range_of_values();
    destructuting_structs();
    destructuring_enums();
    destructuring_nested_structs_and_enums();
    destructuring_complex_values();
    ignoring_values();
    ignoring_unused_variable_warning();
    ignoring_remaining_parts_of_a_value();
    match_guards();
    at_bindings();
}

fn conditional_expressions() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

fn while_let() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    // the while loop continues running the code in its block as long as pop
    // returns Some(value).
    // This is a way to pop every element off our stack.
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn for_loops_pattern() {
    let v = vec!['a', 'b', 'c'];

    // the pattern is the value that directly follows the keyword for
    // ie (index, value)
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

fn let_statements() {
    // Here, x is a pattern
    let x = 5;

    // We can use a pattern to destructure tuples
    let (x, y, z) = (1, 2, 3);

    // This won't compile
    // let (x, y) = (1, 2, 3);
}

fn function_parameters_pattern(x: i32) {
    // the x parameter above is also a pattern

    let point = (3, 5);
    print_coordinates(&point);
    fn print_coordinates(&(x, y): &(i32, i32)) {
        // the values &(3, 5) match the pattern &(x, y) so x is 3 and y is 5
        println!("Current location: ({}, {})", x, y);
    }
}

fn refutability() {
    // Patterns come in 2 forms: refutable and irrefutable.
    // Function parameters, let statements, for loops can only accept
    // irrefutable patterns.

    let some_option_value: Option<i32> = None;
    // Here, x is an irrefutable pattern because it can match Some and None
    let x = some_option_value;

    // This won't compile because we do not cover every valid value
    // let Some(x) = some_option_value;

    // To fix the above example, we have to use if let
    if let Some(x) = some_option_value {
        println!("x = {}", x);
    }
}

fn matching_literals() {
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn matching_named_variables() {
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"), // this won't match
        // This will match because match starts a new scope, so the variable
        // y here is a new variable and not the variable y defined earlier.
        // Here, y will match any value inside a Some value.
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end, x = {:?}, y = {:?}", x, y);
}

fn multiple_patterns() {
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn matching_range_of_values() {
    // The ..= syntax allows us to match to an inclusive range of values

    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // Ranges are only allowed with numeric values or char values, because
    // they are the only types for which Rust can tell if a range is empty or
    // not at compile time.
}

fn destructuting_structs() {
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
    // We can also use the shorthand syntax
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    // We can also destructure with literal values as part of the struct
    // pattern rather than creating variables for all the fields.

    match p {
        // This will match any point on the x axis only if y is 0
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        // This will match any point on the y axis only if x is 0
        Point { x: 0, y } => println!("On the y axis at {}", y),
        // This matches any other Point
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

fn destructuring_enums() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => println!("The Quit variant has no data to destructure."),
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }
}

fn destructuring_nested_structs_and_enums() {
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
        _ => (),
    }
}

fn destructuring_complex_values() {
    struct Point {
        x: i32,
        y: i32,
    }

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
}

fn ignoring_values() {
    //  the underscore (_) is a wildcard pattern that will match any value
    // but not bind to the value.
    // We can use it in any pattern.

    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {}", y);
    }
    foo(3, 4);

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => println!("Some numbers: {}, {}, {}", first, third, fifth),
    }
}

fn ignoring_unused_variable_warning() {
    let _x = 10; // This binds 10 to _y (unlike _ which does not bind the value)
}

fn ignoring_remaining_parts_of_a_value() {
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }
    let origin = Point { x: 0, y: 0, z: 0 };
    match origin {
        // This is quicker than having to list y: _, z:_ , etc.
        Point { x, .. } => println!("x is {}", x),
    }

    // This also works with tuples
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }

    // This won't compile because it's ambiguous
    // let numbers = (2, 4, 8, 16, 32);
    // match numbers {
    //     (.., second, ..) => {
    //         println!("Some numbers: {}", second)
    //     },
    // }
}

fn match_guards() {
    // A match guard is an additional if condition specified after the pattern
    // in a match arm.
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = 4;
    let y = false;
    match x {
        // here, the match guard applies to all patterns, not only 6
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

fn at_bindings() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            // By specifying id_variable @ before the range 3..=7
            // we're capturing whatever value matched the range
            // while also testing that the value matched the range pattern
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        // Here, we don't have a variable that contains the actual value of
        // the id field
        Message::Hello { id: 10..=12 } => println!("Found an id in another range"),
        // Here, we do have the value available to use in a variable
        // named id, because we're using the shorthand syntax
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}
