// A trait is a collection of methods defined for an unknown type: Self
// Traits can be implemented for any data type

trait Animal {
    // static method
    fn new(age: u32) -> Self;
    // method for the implementor type - &self as first parameter
    fn noise(&self) -> &str;
    // Traits can provide default method definitions
    fn talk(&self) {
        println!("...");
    }
}

struct Dog {
    age: u32,
}

impl Dog {
    fn jump(&self) {
        println!("*dog jumps*");
    }
}

// Implement the Animal trait for the Dog data type
impl Animal for Dog {
    fn new(age: u32) -> Dog {
        Dog { age }
    }
    fn noise(&self) -> &str {
        "woof"
    }
    // default trait methods can be overriden
    fn talk(&self) {
        println!("{}", self.noise().repeat(self.age as usize));
    }
}

pub fn example() {
    let dog = Dog::new(2);
    dog.talk(); // "woofwoof"
    dog.jump(); // "*dog jumps*"

    // Type annotation is necessary in this case
    let puppy: Dog = Animal::new(1);
    puppy.talk(); // "woof"
}
