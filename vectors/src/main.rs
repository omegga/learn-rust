fn main() {
    vectors();
    initializing_a_vector();
    adding_elements();
    reading_elements();
    invalid_element_access();
    invalid_element_access_2();
    borrowing();
    iterating();
    storing_different_types();
}

fn vectors() {
    // A Vector is a collection type that stores multiple values of the same
    // type, next to each other.
    // This creates a new, empty vector:

    let v: Vec<i32> = Vec::new();
    println!("v = {:?}", v);

    // Vectors are implemented using generics and can hold any type.
    // In this function block, since we won't add elements to v, we have to
    // specify the type because Rust cannot infer the type of an empty vector.
}

fn initializing_a_vector() {
    // It is more common to create a vector with inital values, using the vec!
    // macro:
    let v = vec![1, 2, 3]; // Here, Rust can infer the type of v (ie Vec<i32>)
    println!("v = {:?}", v);
}

fn adding_elements() {
    // We can use the push method to add elements:
    // (as with any variable that can change its value, we need to use the mut
    // keyword)

    let mut v = Vec::new(); // Rust will infer the type of v from the next lines
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);

    // The vector is freed when it goes out of scope.
    // Dropping a vector drops its elements as well.
}

fn reading_elements() {
    let v = vec![1, 2, 3];
    // There are 2 ways to reference a value stored in a vector.
    // Using an index
    let third: &i32 = &v[2];
    println!("third = {}", third);
    // Using the get method (which returns an Option<&T>)
    match v.get(2) {
        Some(third) => println!("third = {}", third),
        None => println!("no third element"),
    }
}

fn invalid_element_access() {
    let _v = vec![1, 2, 3];

    // The code below panics
    // --> this is useful when this errors should not happen or we want to our program
    // to crash.
    // let does_not_exist = &_v[100];
}

fn invalid_element_access_2() {
    let v = vec![1, 2, 3];
    // The code will return None
    // --> this is useful when it happens occasionnaly under normal circumstances
    let does_not_exist = v.get(100);
    assert_eq!(None, does_not_exist);
}

fn borrowing() {
    // The code below won't compile

    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6);
    // println!("first = {}", first);

    // Since vectors requires values to be next to each other, adding a new
    // element to a vector might require allocating new memory and copying the
    // old elements to the new space.

    // In the code above, this would mean "first" would be pointing to deallocated
    // memory.
}

fn iterating() {
    // We can iterate over immutable references of a vector
    let v = vec![1, 2, 3];
    for i in &v {
        println!("i = {}", i);
    }

    // We can also iterate over mutable references of a vector
    // (To change all its elements, for example)
    let mut v = vec![1, 2, 3];
    for i in &mut v {
        *i += 1;
    }
    println!("v = {:?}", v);
}

fn storing_different_types() {
    // Vectors can only store elements of the same type.
    // But using enums, we can define values that will hold different types:
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
