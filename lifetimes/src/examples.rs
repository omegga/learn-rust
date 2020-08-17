// Error: missing lifetime specifier
// a gets dropped at the end of this function
// pub fn get_int_ref() -> &i32 {
//     let a = 1;
//     &a
// }

// OK
// here we return the ownership
pub fn get_int_ref_2() -> i32 {
    let a = 1;
    a
}

// OK
// the scope that provides the reference is the same exact scope
// that will be receiving the result output
pub fn get_int_ref_3(param_1: &i32) -> &i32 {
    &param_1
}

// Rust automatacily creates lifetimes for function parameters
// that are references
// get_int_ref_3 could be written as:
pub fn get_int_ref_4<'a>(param_1: &'a i32) -> &'a i32 {
    // the "a" in 'a is a convention, we could give any name to a lifetime
    // the input memory exists in the same scope as the output memory
    &param_1
}

// OK
pub fn get_int_ref_5<'a, 'b>(param_1: &'a i32, param_2: &'b i32) -> &'a i32 {
    println!("{}", param_2);
    &param_1
}

// Error
// lifetime mismatch
// We return a reference with the wrong lifetime
// pub fn get_int_ref_6<'a, 'b>(param_1: &'a i32, param_2: &'b i32) -> &'a i32 {
//     &param_2 // lifetime mismatch
// }

// Error
// Here the compiler has to know what lifetime ('a or 'b ?) to assign the output
// pub fn get_int_ref_7<'a, 'b>(param_1: &'a i32, param_2: &'b i32) -> &i32 {
//     &param_1 // lifetime mismatch
// }
