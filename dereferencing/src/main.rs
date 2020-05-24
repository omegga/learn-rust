fn main() {
    // Smart pointers are data structures that act like a pointer
    // but also have additional metadata and capabilities

    // References (the most common kind of pointer, indicated by the & symbol)
    // only borrow data

    // In many cases, smart pointers own the data they point to

    // Examples of smart pointers: String, Vec<T>,...
    // They own some memory and allow you to manipulate it

    // Smart pointers are usually implemented using structs.
    // The difference between a smart pointer and an ordinary struct
    // is that smart pointers implement the Deref and Drop traits

    following_the_pointer();
    not_working();
    using_a_box();
    creating_a_smart_pointer();
}

fn following_the_pointer() {
    let x = 5; // the variable x holds an i32 value
    let y = &x; // we set y equal to a reference to x
    assert_eq!(5, x);
    // using *y, we follow the reference to the value it's pointing to (hence "dereference").
    // Once we dereference y, we have access to the integer value y is pointing to
    // and we can compare with 5
    assert_eq!(5, *y);
}

fn not_working() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    // Comparing a number and a reference to a number isn't allowed because they're different types
    // assert_eq!(5, y); // compilation error
}

fn using_a_box() {
    let x = 5;
    // Here we set y to be an instance of a box pointing to the value in x
    // rather than a reference pointing to the value of x
    let y = Box::new(x);
    println!("x = {}", x);
    println!("*y = {}", *y);
}

fn creating_a_smart_pointer() {
    #[derive(Debug)]
    struct MyBox<T>(T);
    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }
    use std::ops::Deref;
    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &T {
            &self.0
        }
    }
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
