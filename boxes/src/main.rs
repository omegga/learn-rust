fn main() {
    creating_a_box();
    not_working();
    creating_a_list();
}

fn creating_a_box() {
    let b = Box::new(5);
    println!("b = {}", b);
}

fn not_working() {
    // this won't compile due to infinite size
    // enum List {
    //     Cons(i32, List),
    //     Nil,
    // }
    // let list = Cons(1, Cons(2, Cons(3, Nil)));
}

fn creating_a_list() {
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    use List::{Cons, Nil};
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("list = {:?}", list);
    // Box<T> implements the Deref trait (allows it to be treated like a reference)
    // and the Drop trait (if it goes out of scope, the heap data is cleaned up as well)
}
