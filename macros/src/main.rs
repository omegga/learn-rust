// Macros are a way of writing code that writes other code

macro_rules! my_macro {
    // there must be at least one rule
    () => {
        println!("Hello, world!")
    };
}

#[macro_use] // this exposes the macro to a wider scope
mod macros {
    // the macro_rules! interpreter goes through the rules one by one,
    // in lexical order.
    // For each rule, it tries to match the contents of the input token tree
    // against that rule's pattern.
    // A pattern must match the entirety fof the input to be considered a match
    macro_rules! my_inner_macro {
        // this matches only if the input is also empty, like so:
        // my_fantastic_macro!()
        // my_fantastic_macro![]
        // my_fantastic_macro!{}
        () => {
            println!("Check out my inner macro!");
        };
    }
}

macro_rules! my_fantastic_macro {
    () => {
        println!("Check out my fantastic macro!");
    }; // macro rules are separated by ";"
    // each rule looks like so: ($pattern) => {$expansion}
    // Captures are written as a dollar($) followed by an identifier (val),
    // a colon (:), and finally the kind of capture (expr, for an expression)
    ($val:expr) => {
        println!("Look at this other fantastic macro: {}", $val);
    }; // the semicolon after last rule can be omitted
}

fn main() {
    // macros must be called with an exclamation mark
    my_macro!();
    // not_working_macro!();
    my_inner_macro!(); // "Check out my inner macro!"
    my_fantastic_macro!(); // "Check out my fantastic macro!"
    my_fantastic_macro!(7777); // Look at this other fantastic macro: 7777"
}

// Macros must be defined (or brought in scope) before you call them in a file
// macro_rules! not_working_macro {
//     () => {
//         println!("Hello")
//     };
// }
