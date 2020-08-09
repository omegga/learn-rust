use std::error::Error;
// use std::fs::File;

// Many different operations could go wrong
// Box<dyn Error> ensures we can return any error
pub fn example() -> Result<(), Box<dyn Error>> {
    "7777".parse::<i32>()?;
    // "beep boop".parse::<i32>()?;
    // let f = File::open("hello.txt")?;
    Ok(())
}
