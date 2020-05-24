fn main() {
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
