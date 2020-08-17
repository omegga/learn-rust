pub fn example_1() {
    let a;
    {
        let b = String::from("Hello");
        a = b; // a takes ownership of b value
    }
    println!("{}", a); // this works fine
}

// pub fn example_2() {
//     let a;
//     {
//         let b = String::from("Hello");
//         a = &b; // b does not live long enough because it's dropped after this line
//     }
//     println!("{}", a);
// }
